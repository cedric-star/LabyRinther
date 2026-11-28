// Host-Seite des Multiplayers.
//
// Der Host ist der "Listen Server": er spielt selbst mit UND simuliert die
// Welt für alle (Gegner, Wellen, Schaden). Clients schicken ihren eigenen
// Spielerzustand, der Host hält dafür "Geister" (RemotePlayer) nach.

use std::collections::{HashMap, VecDeque};
use std::io;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::*;
use bevy::ui::*;
use avian2d::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::ingame::animation::*;
use crate::ingame::entity::enemy::Enemy;
use crate::ingame::entity::structs::Health;
use crate::ingame::items::attack::structs::*;
use crate::ingame::player::structs::*;
use crate::ingame::structs::Layer;
use crate::resources::WorldSeed;
use crate::net::protocol::*;
use crate::net::*;

/// Der Host selbst ist Spieler mit der ID 0.
pub const HOST_PLAYER_ID: u64 = 0;

// ---------------------------------------------------------------------
// Ressourcen & Verbindung
// ---------------------------------------------------------------------

#[derive(Resource)]
pub struct NetServer {
    /// Lauscht auf neue Verbindungen (non-blocking, wird pro Frame gepollt).
    listener: TcpListener,
    /// Dürfen gerade neue Spieler beitreten? (im Spiel mit F10 umschaltbar)
    pub allow_joins: bool,
    pub clients: HashMap<u64, NetConnection>,
    /// Posteingang der Reader-Threads.
    inbox: Arc<Mutex<VecDeque<ServerEvent>>>,
    next_conn_id: u64,
    next_player_id: u64,
    /// Bestmögliche Schätzung der LAN-IP (für die Anzeige).
    pub lan_ip: String,
}

pub struct NetConnection {
    pub conn_id: u64,
    /// gesetzt, sobald der Join akzeptiert wurde
    pub player_id: Option<u64>,
    pub stream: TcpStream,
}

pub enum ServerEvent {
    Message { conn_id: u64, msg: ClientMessage },
    Disconnected { conn_id: u64 },
}

impl NetServer {

    pub fn start(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(("0.0.0.0", port))?;
        listener.set_nonblocking(true)?;
        Ok(Self {
            listener,
            allow_joins: true,
            clients: HashMap::new(),
            inbox: Arc::new(Mutex::new(VecDeque::new())),
            next_conn_id: 1,
            next_player_id: 1, // der Host selbst ist Spieler 0
            lan_ip: local_lan_ip().unwrap_or_else(|| "?".to_string()),
        })
    }

    /// Nachricht an alle verbundenen Clients. Liefert fehlgeschlagene Verbindungen.
    fn broadcast(&mut self, msg: &ServerMessage) -> Vec<u64> {
        let mut failed = Vec::new();
        for (id, conn) in self.clients.iter_mut() {
            if write_message(&mut conn.stream, msg).is_err() {
                failed.push(*id);
            }
        }
        failed
    }
}

fn local_lan_ip() -> Option<String> {
    // Trick: ein UDP-Socket "verbinden" - es wird nichts gesendet,
    // aber das Routing verrät uns die lokale Adresse.
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}

/// Liest in einem eigenen Thread alle Nachrichten einer Verbindung.
fn reader_thread(mut stream: TcpStream, conn_id: u64, inbox: Arc<Mutex<VecDeque<ServerEvent>>>) {
    loop {
        let event = match read_message::<ClientMessage>(&mut stream) {
            Ok(Some(msg)) => ServerEvent::Message { conn_id, msg },
            _ => ServerEvent::Disconnected { conn_id },
        };
        let is_disconnect = matches!(event, ServerEvent::Disconnected { .. });
        let mut queue = inbox.lock().unwrap();
        if queue.len() > 1024 { break; } // Schutz vor Spam
        queue.push_back(event);
        drop(queue);
        if is_disconnect { break; }
    }
}

// ---------------------------------------------------------------------
// Systeme
// ---------------------------------------------------------------------

/// Nimmt neue Verbindungen an (nicht-blockierend, einmal pro Frame).
pub fn server_accept_new_connections(mut server: ResMut<NetServer>) {
    loop {
        match server.listener.accept() {
            Ok((mut stream, _addr)) => {
                let _ = stream.set_nonblocking(false);
                let _ = stream.set_nodelay(true);
                let _ = stream.set_write_timeout(Some(Duration::from_millis(200)));

                if !server.allow_joins {
                    let _ = write_message(&mut stream, &ServerMessage::JoinDenied {
                        reason: "Der Host erlaubt gerade keine Beitritte".to_string(),
                    });
                    let _ = stream.shutdown(Shutdown::Both);
                    continue;
                }

                let conn_id = server.next_conn_id;
                server.next_conn_id += 1;
                if let Ok(reader) = stream.try_clone() {
                    let inbox = server.inbox.clone();
                    thread::spawn(move || reader_thread(reader, conn_id, inbox));
                    server.clients.insert(conn_id, NetConnection { conn_id, player_id: None, stream });
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) => { println!("Netzwerk accept-Fehler: {e}"); break; }
        }
    }
}

/// Verarbeitet alle eingegangenen Client-Nachrichten (Join, Zustand, Angriffe).
pub fn server_handle_messages(
    mut commands: Commands,
    mut server: ResMut<NetServer>,
    seed: Res<WorldSeed>,
    mut remote_players: ResMut<RemotePlayers>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    host_players: Query<Entity, With<Player>>,
    children: Query<&Children>,
    player_bodies: Query<Entity, With<PlayerBody>>,
    enemies: Query<(Entity, &Health, &Transform), With<Enemy>>,
    mut anims: Query<&mut Anim>,
    mut attackers: Query<(&Transform, &mut Attacker, Option<&AttackTracker>)>,
    mut ghost_targets: Query<&mut NetTarget, With<RemotePlayer>>,
) {
    let inbox = server.inbox.clone();
    let mut failed: Vec<u64> = Vec::new();
    let mut queue = inbox.lock().unwrap();

    while let Some(event) = queue.pop_front() {
        let conn_id = match event {
            ServerEvent::Message { conn_id, .. } => conn_id,
            ServerEvent::Disconnected { conn_id } => {
                failed.push(conn_id);
                continue;
            }
        };
        let msg = match event {
            ServerEvent::Message { msg, .. } => msg,
            ServerEvent::Disconnected { .. } => unreachable!(),
        };

        match msg {
            ClientMessage::JoinRequest { protocol } => {
                if server.clients.get(&conn_id).is_some_and(|c| c.player_id.is_some()) {
                    continue; // schon gejoint
                }
                if protocol != PROTOCOL_VERSION {
                    if let Some(conn) = server.clients.get_mut(&conn_id) {
                        let _ = write_message(&mut conn.stream, &ServerMessage::JoinDenied {
                            reason: format!("Protokoll-Version passt nicht (Server: {PROTOCOL_VERSION}, Client: {protocol})"),
                        });
                    }
                    failed.push(conn_id);
                    continue;
                }

                let player_id = server.next_player_id;
                server.next_player_id += 1;

                // aktuellen Weltzustand einsammeln und als Welcome schicken
                let mut players: Vec<NetPlayer> = Vec::new();
                if let Ok(host_entity) = host_players.single() {
                    if let Some(body) = find_child_body(&children, &player_bodies, host_entity) {
                        players.push(net_player_for(HOST_PLAYER_ID, host_entity, body, &mut attackers, &mut anims));
                    }
                }
                for (id, entry) in remote_players.0.iter() {
                    players.push(net_player_for(*id, entry.root, entry.body, &mut attackers, &mut anims));
                }
                let enemy_list = collect_enemies(&enemies, &mut anims);

                let Some(conn) = server.clients.get_mut(&conn_id) else { continue };
                conn.player_id = Some(player_id);
                if write_message(&mut conn.stream, &ServerMessage::Welcome {
                    seed: seed.seed,
                    your_id: player_id,
                    players,
                    enemies: enemy_list,
                }).is_err() {
                    failed.push(conn_id);
                }
            }

            ClientMessage::PlayerState { pos, anim } => {
                let Some(player_id) = server.clients.get(&conn_id).and_then(|c| c.player_id) else { continue };
                let pos = Vec2::new(pos.0, pos.1);

                let existing = remote_players.0.get(&player_id).copied();
                match existing {
                    Some(entry) => {
                        if let Ok(mut target) = ghost_targets.get_mut(entry.root) { target.0 = pos; }
                        if let Ok(mut a) = anims.get_mut(entry.body) { *a = Anim::Humanoid(anim); }
                    }
                    None => {
                        // erster Zustand dieses Spielers -> Geist erzeugen
                        let entry = spawn_remote_player_host(&mut commands, &asset_server, &mut layouts, player_id, pos, anim);
                        remote_players.0.insert(player_id, entry);
                    }
                }
            }

            ClientMessage::Attack { item, aim } => {
                let Some(player_id) = server.clients.get(&conn_id).and_then(|c| c.player_id) else { continue };
                let Some(entry) = remote_players.0.get(&player_id).copied() else { continue };
                if let Ok((_, mut attacker, _)) = attackers.get_mut(entry.root) {
                    attacker.active = Some(ActiveAttack {
                        item,
                        elapsed: 0.0,
                        hit_fired: false,
                        aim: Vec2::new(aim.0, aim.1),
                        hit_entities: Vec::new(),
                    });
                }
            }
        }
    }
    drop(queue);

    for conn_id in failed {
        disconnect_client(&mut commands, &mut server, &mut remote_players, conn_id);
    }
}

/// Schickt ~20x pro Sekunde den Weltzustand an alle Clients.
pub fn server_send_snapshots(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ServerSnapshotTimer>,
    mut server: ResMut<NetServer>,
    mut remote_players: ResMut<RemotePlayers>,
    host_players: Query<Entity, With<Player>>,
    children: Query<&Children>,
    player_bodies: Query<Entity, With<PlayerBody>>,
    enemies: Query<(Entity, &Health, &Transform), With<Enemy>>,
    mut anims: Query<&mut Anim>,
    mut attackers: Query<(&Transform, &mut Attacker, Option<&AttackTracker>)>,
) {
    if !timer.0.tick(time.delta()).just_finished() { return; }

    let mut players: Vec<NetPlayer> = Vec::new();
    if let Ok(host_entity) = host_players.single() {
        if let Some(body) = find_child_body(&children, &player_bodies, host_entity) {
            players.push(net_player_for(HOST_PLAYER_ID, host_entity, body, &mut attackers, &mut anims));
        }
    }
    for (id, entry) in remote_players.0.iter() {
        players.push(net_player_for(*id, entry.root, entry.body, &mut attackers, &mut anims));
    }
    let enemy_list = collect_enemies(&enemies, &mut anims);

    let failed = server.broadcast(&ServerMessage::Snapshot { players, enemies: enemy_list });
    for conn_id in failed {
        disconnect_client(&mut commands, &mut server, &mut remote_players, conn_id);
    }
}

/// Erkennt, wann ein Spieler (Host oder Geist) einen NEUEN Angriff startet,
/// und zählt einen Sequenz-Zähler hoch. Der geht in die Snapshots.
pub fn track_attack_seqs(
    mut commands: Commands,
    query: Query<(Entity, &Attacker), Or<(With<Player>, With<RemotePlayer>)>>,
    mut trackers: Query<&mut AttackTracker>,
) {
    for (entity, attacker) in &query {
        if let Ok(mut tracker) = trackers.get_mut(entity) {
            match &attacker.active {
                Some(active) => {
                    // elapsed ist bei einem neuen Angriff wieder auf ~0 zurückgesprungen
                    if tracker.last_elapsed.map_or(true, |old| active.elapsed < old) {
                        tracker.seq += 1;
                    }
                    tracker.last_elapsed = Some(active.elapsed);
                }
                None => tracker.last_elapsed = None,
            }
        } else {
            commands.entity(entity).insert(AttackTracker::default());
        }
    }
}

/// Zieht die Spieler-Geister auf dem Host zur Netzwerk-Position.
/// (Physik-Körper, damit Waffen-Kollisionen und Gegner-Blockade funktionieren.)
pub fn drive_remote_player_bodies(
    mut commands: Commands,
    mut remote_players: ResMut<RemotePlayers>,
    mut ghosts: Query<(Entity, &NetPlayerId, &Transform, &NetTarget, &mut LinearVelocity), With<RemotePlayer>>,
) {
    let mut respawn: Vec<u64> = Vec::new();
    for (entity, id, tf, target, mut vel) in &mut ghosts {
        let delta = target.0 - tf.translation.truncate();
        if delta.length() > 800.0 {
            // hoffnungslos hängen geblieben -> neu spawnen beim nächsten Zustand
            respawn.push(id.0);
            commands.entity(entity).despawn();
            continue;
        }
        vel.0 = (delta * 10.0).clamp_length_max(1500.0);
    }
    for id in respawn {
        remote_players.0.remove(&id);
    }
}

// ---------------------------------------------------------------------
// Server-Status Anzeige (Beitritte ein/aus)
// ---------------------------------------------------------------------

#[derive(Component)]
pub struct ServerStatusText;

pub fn spawn_server_status_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::InGame),
        ServerStatusText,
        Node {
            position_type: PositionType::Absolute,
            top: px(8.0),
            left: px(8.0),
            ..default()
        },
        Text::new(""),
        TextFont { font_size: FontSize::Px(20.0), ..default() },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
    ));
}

pub fn server_toggle_joins(keys: Res<ButtonInput<KeyCode>>, mut server: ResMut<NetServer>) {
    if keys.just_pressed(KeyCode::F10) {
        server.allow_joins = !server.allow_joins;
        println!("Server: Beitritte sind jetzt {}", if server.allow_joins { "OFFEN" } else { "GESPERRT" });
    }
}

pub fn update_server_status_text(server: Res<NetServer>, mut texts: Query<&mut Text, With<ServerStatusText>>) {
    let joined = server.clients.values().filter(|c| c.player_id.is_some()).count();
    let text = format!(
        "Host {}:{} | Beitritte: {} [F10] | Spieler online: {}",
        server.lan_ip,
        DEFAULT_PORT,
        if server.allow_joins { "OFFEN" } else { "GESPERRT" },
        joined + 1,
    );
    for mut t in &mut texts {
        if t.0 != text { t.0 = text.clone(); }
    }
}

// ---------------------------------------------------------------------
// Hilfsfunktionen
// ---------------------------------------------------------------------

fn disconnect_client(
    commands: &mut Commands,
    server: &mut NetServer,
    remote_players: &mut RemotePlayers,
    conn_id: u64,
) {
    let Some(conn) = server.clients.remove(&conn_id) else { return };
    let _ = conn.stream.shutdown(Shutdown::Both);
    if let Some(player_id) = conn.player_id {
        if let Some(entry) = remote_players.0.remove(&player_id) {
            commands.entity(entry.root).despawn();
        }
    }
}

fn find_child_body(
    children: &Query<&Children>,
    bodies: &Query<Entity, With<PlayerBody>>,
    root: Entity,
) -> Option<Entity> {
    children.iter_descendants(root).find(|&c| bodies.get(c).is_ok())
}

fn net_player_for(
    id: u64,
    root: Entity,
    body: Entity,
    attackers: &mut Query<(&Transform, &mut Attacker, Option<&AttackTracker>)>,
    anims: &mut Query<&mut Anim>,
) -> NetPlayer {
    let Ok((tf, attacker, tracker)) = attackers.get_mut(root) else {
        return NetPlayer { id, pos: (0.0, 0.0), anim: HumanoidState::StandingFront, attack: None };
    };
    let anim = anims
        .get_mut(body)
        .map(|a| match &*a { Anim::Humanoid(s) => *s })
        .unwrap_or(HumanoidState::StandingFront);
    let attack = attacker.active.as_ref().map(|a| NetAttack {
        seq: tracker.map_or(0, |t| t.seq),
        item: a.item,
        aim: (a.aim.x, a.aim.y),
    });
    NetPlayer { id, pos: (tf.translation.x, tf.translation.y), anim, attack }
}

fn collect_enemies(
    enemies: &Query<(Entity, &Health, &Transform), With<Enemy>>,
    anims: &mut Query<&mut Anim>,
) -> Vec<NetEnemy> {
    enemies
        .iter()
        .map(|(entity, health, tf)| {
            let anim = anims
                .get_mut(entity)
                .map(|a| match &*a { Anim::Humanoid(s) => *s })
                .unwrap_or(HumanoidState::StandingFront);
            NetEnemy {
                id: entity.to_bits(),
                pos: (tf.translation.x, tf.translation.y),
                anim,
                health: health.health_points,
                max_health: health.max_health,
            }
        })
        .collect()
}

/// Geist eines Client-Spielers auf dem Host: gleicher Aufbau wie der echte
/// Player (Physik + Attacker + Waffen-Sensor), damit alle bestehenden
/// Angriffs-Systeme unverändert damit arbeiten können.
fn spawn_remote_player_host(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    id: u64,
    pos: Vec2,
    anim: HumanoidState,
) -> RemotePlayerEntry {
    let clips = Player::get_clips(asset_server, layouts);
    let clip = clips.clips.get(&Anim::Humanoid(anim)).cloned().unwrap();

    let root = commands.spawn((
        RemotePlayer,
        NetPlayerId(id),
        NetTarget(pos),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::circle(PLAYER_WIDTH / 2.0),
        CollisionLayers::new(Layer::Player, [Layer::Wall, Layer::Enemy, Layer::EnemyWeapon]),
        Mass(10000000.0),
        Transform::from_xyz(pos.x, pos.y, ZLayer::Player.z()),
        Visibility::default(),
        Attacker::default(),
    )).id();

    let body = commands.spawn((
        ChildOf(root),
        RemotePlayerBody,
        Visibility::default(),
        Anim::Humanoid(anim),
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: clip.image.clone(),
            texture_atlas: Some(TextureAtlas { layout: clip.layout.clone(), index: clip.first }),
            ..default()
        },
        clips,
        AnimationTimer::new(),
        Transform::default(),
    )).id();

    let pivot = commands.spawn((
        ChildOf(root),
        HandPivot,
        Visibility::default(),
        Transform::default(),
    )).id();

    commands.spawn((
        ChildOf(pivot),
        HeldItemSprite,
        Sprite::default(),
        Anchor::BOTTOM_LEFT,
        Visibility::Hidden,
        Sensor,
        Collider::compound(vec![(Vec2::new(16.0, 16.0), 0.0, Collider::rectangle(32.0, 32.0))]),
        ColliderDisabled,
        CollisionLayers::new(Layer::PlayerWeapon, [Layer::Enemy]),
        CollidingEntities::default(),
    ));

    RemotePlayerEntry { root, body, attack_seq: 0 }
}

