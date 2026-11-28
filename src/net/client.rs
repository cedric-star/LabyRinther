// Client-Seite des Multiplayers.
//
// Der Client simuliert nur seinen eigenen Spieler und die (aus dem Seed
// deterministisch generierte) Welt. Gegner und andere Spieler sind Geister,
// deren Position aus den Snapshots interpoliert wird.

use std::collections::{HashSet, VecDeque};
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::*;
use avian2d::prelude::ColliderDisabled;

use crate::components::*;
use crate::constants::*;
use crate::ingame::animation::*;
use crate::ingame::entity::enemy::*;
use crate::ingame::entity::structs::Health;
use crate::ingame::items::attack::structs::*;
use crate::ingame::player::structs::*;
use crate::resources::WorldSeed;
use crate::net::join_ui::JoinStatus;
use crate::net::protocol::*;
use crate::net::*;

#[derive(Resource)]
pub struct NetClient {
    pub writer: TcpStream,
    inbox: Arc<Mutex<VecDeque<ClientEvent>>>,
    pub my_player_id: u64,
}

pub enum ClientEvent {
    Message(ServerMessage),
    Disconnected,
}

impl NetClient {
    pub fn connect(stream: TcpStream) -> Result<Self, String> {
        let _ = stream.set_nodelay(true);
        let _ = stream.set_write_timeout(Some(Duration::from_millis(200)));
        let writer = stream.try_clone().map_err(|e| format!("{e}"))?;
        let (tx_ok, inbox) = ((), Arc::new(Mutex::new(VecDeque::new())));
        let _ = tx_ok;
        let reader = stream;
        let inbox_writer = inbox.clone();
        thread::spawn(move || client_reader_thread(reader, inbox_writer));
        Ok(Self { writer, inbox, my_player_id: 0 })
    }

    pub fn send(&mut self, msg: &ClientMessage) {
        if let Err(e) = write_message(&mut self.writer, msg) {
            println!("Netzwerk-Sendefehler: {e}");
        }
    }
}

fn client_reader_thread(mut stream: TcpStream, inbox: Arc<Mutex<VecDeque<ClientEvent>>>) {
    loop {
        let event = match read_message::<ServerMessage>(&mut stream) {
            Ok(Some(msg)) => ClientEvent::Message(msg),
            _ => ClientEvent::Disconnected,
        };
        let is_disconnect = matches!(event, ClientEvent::Disconnected);
        let mut queue = inbox.lock().unwrap();
        if queue.len() > 1024 { break; }
        queue.push_back(event);
        drop(queue);
        if is_disconnect { break; }
    }
}

// ---------------------------------------------------------------------
// Systeme
// ---------------------------------------------------------------------

/// Verarbeitet alle Nachrichten vom Host.
pub fn client_handle_messages(
    mut commands: Commands,
    mut client: ResMut<NetClient>,
    mut remote_players: ResMut<RemotePlayers>,
    mut remote_enemies: ResMut<RemoteEnemies>,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut ghost_targets: Query<&mut NetTarget, Or<(With<RemotePlayer>, With<RemoteEnemy>)>>,
    mut anims: Query<&mut Anim>,
    mut attackers: Query<&mut Attacker, With<RemotePlayer>>,
    mut healths: Query<&mut Health, With<RemoteEnemy>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut join_status: ResMut<JoinStatus>,
) {
    let inbox = client.inbox.clone();
    let mut queue = inbox.lock().unwrap();

    while let Some(event) = queue.pop_front() {
        match event {
            ClientEvent::Message(ServerMessage::JoinDenied { reason }) => {
                drop(queue);
                join_status.0 = format!("Beitritt abgelehnt: {reason}");
                let _ = client.writer.shutdown(Shutdown::Both);
                commands.remove_resource::<NetClient>();
                return;
            }
            ClientEvent::Message(ServerMessage::Welcome { seed, your_id, players, enemies }) => {
                client.my_player_id = your_id;
                // gleiche Welt lokal aus dem Seed generieren
                commands.insert_resource(WorldSeed { seed });
                apply_player_list(&players, your_id, &mut commands, &asset_server, &mut layouts,
                    &mut remote_players, &mut ghost_targets, &mut anims, &mut attackers);
                apply_enemy_list(&enemies, &mut commands, &asset_server, &mut layouts,
                    &mut remote_enemies, &mut ghost_targets, &mut anims, &mut healths);
                next_state.set(GameState::InGame);
            }
            ClientEvent::Message(ServerMessage::Snapshot { players, enemies }) => {
                let my_id = client.my_player_id;
                apply_player_list(&players, my_id, &mut commands, &asset_server, &mut layouts,
                    &mut remote_players, &mut ghost_targets, &mut anims, &mut attackers);
                apply_enemy_list(&enemies, &mut commands, &asset_server, &mut layouts,
                    &mut remote_enemies, &mut ghost_targets, &mut anims, &mut healths);
            }
            ClientEvent::Disconnected => {
                drop(queue);
                println!("Verbindung zum Host verloren");
                let _ = client.writer.shutdown(Shutdown::Both);
                commands.remove_resource::<NetClient>();
                if *state.get() == GameState::InGame {
                    next_state.set(GameState::Welcome);
                } else {
                    join_status.0 = "Verbindung zum Host verloren".to_string();
                }
                return;
            }
        }
    }
}

/// Schickt ~20x pro Sekunde den eigenen Spieler an den Host.
pub fn client_send_state(
    time: Res<Time>,
    mut timer: ResMut<ClientSendTimer>,
    mut client: ResMut<NetClient>,
    player: Single<&Transform, With<Player>>,
    anim: Single<&Anim, With<PlayerBody>>,
) {
    if !timer.0.tick(time.delta()).just_finished() { return; }
    let anim_state = match **anim { Anim::Humanoid(s) => s };
    client.send(&ClientMessage::PlayerState {
        pos: (player.translation.x, player.translation.y),
        anim: anim_state,
    });
}

/// Bewegt alle Geister weich Richtung Netzwerk-Position.
pub fn interpolate_remotes(
    time: Res<Time>,
    mut ghosts: Query<(&mut Transform, &NetTarget), Or<(With<RemotePlayer>, With<RemoteEnemy>)>>,
) {
    let t = (time.delta_secs() * 12.0).min(1.0);
    for (mut tf, target) in &mut ghosts {
        let next = tf.translation.truncate().lerp(target.0, t);
        tf.translation.x = next.x;
        tf.translation.y = next.y;
    }
}

// ---------------------------------------------------------------------
// Geister-Verwaltung
// ---------------------------------------------------------------------

fn apply_player_list(
    players: &[NetPlayer],
    my_id: u64,
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    remote_players: &mut RemotePlayers,
    ghost_targets: &mut Query<&mut NetTarget, Or<(With<RemotePlayer>, With<RemoteEnemy>)>>,
    anims: &mut Query<&mut Anim>,
    attackers: &mut Query<&mut Attacker, With<RemotePlayer>>,
) {
    let mut seen: HashSet<u64> = HashSet::new();

    for p in players {
        if p.id == my_id { continue; } // eigener Spieler
        seen.insert(p.id);

        let pos = Vec2::new(p.pos.0, p.pos.1);
        let existing = remote_players.0.get(&p.id).copied();

        match existing {
            Some(entry) => {
                if let Ok(mut target) = ghost_targets.get_mut(entry.root) { target.0 = pos; }
                if let Ok(mut a) = anims.get_mut(entry.body) { *a = Anim::Humanoid(p.anim); }

                // neuen Angriff lokal (rein visuell) abspielen
                if let Some(atk) = &p.attack {
                    if atk.seq > entry.attack_seq {
                        if let Some(map_entry) = remote_players.0.get_mut(&p.id) {
                            map_entry.attack_seq = atk.seq;
                        }
                        if let Ok(mut attacker) = attackers.get_mut(entry.root) {
                            attacker.active = Some(ActiveAttack {
                                item: atk.item,
                                elapsed: 0.0,
                                hit_fired: false,
                                aim: Vec2::new(atk.aim.0, atk.aim.1),
                                hit_entities: Vec::new(),
                            });
                        }
                    }
                }
            }
            None => {
                let entry = spawn_remote_player_client(commands, asset_server, layouts, p.id, pos, p.anim);
                remote_players.0.insert(p.id, entry);
            }
        }
    }

    // Spieler entfernen, die nicht mehr im Snapshot sind
    let gone: Vec<u64> = remote_players.0.keys().filter(|id| !seen.contains(id)).copied().collect();
    for id in gone {
        if let Some(entry) = remote_players.0.remove(&id) {
            commands.entity(entry.root).despawn();
        }
    }
}

fn apply_enemy_list(
    enemies: &[NetEnemy],
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    remote_enemies: &mut RemoteEnemies,
    ghost_targets: &mut Query<&mut NetTarget, Or<(With<RemotePlayer>, With<RemoteEnemy>)>>,
    anims: &mut Query<&mut Anim>,
    healths: &mut Query<&mut Health, With<RemoteEnemy>>,
) {
    let mut seen: HashSet<u64> = HashSet::new();

    for e in enemies {
        seen.insert(e.id);
        let pos = Vec2::new(e.pos.0, e.pos.1);

        match remote_enemies.0.get(&e.id).copied() {
            Some(entity) => {
                if let Ok(mut target) = ghost_targets.get_mut(entity) { target.0 = pos; }
                if let Ok(mut a) = anims.get_mut(entity) { *a = Anim::Humanoid(e.anim); }
                if let Ok(mut h) = healths.get_mut(entity) {
                    h.health_points = e.health;
                    h.max_health = e.max_health;
                }
            }
            None => {
                let entity = spawn_remote_enemy_client(commands, asset_server, layouts, e.id, pos, e.anim, e.health, e.max_health);
                remote_enemies.0.insert(e.id, entity);
            }
        }
    }

    let gone: Vec<u64> = remote_enemies.0.keys().filter(|id| !seen.contains(id)).copied().collect();
    for id in gone {
        if let Some(entity) = remote_enemies.0.remove(&id) {
            commands.entity(entity).despawn();
        }
    }
}

/// Rein visuelles Spieler-Abbild (keine Physik - Schaden berechnet nur der Host).
fn spawn_remote_player_client(
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

    // nur der Sprite, kein Collider: Treffer berechnet allein der Host
    commands.spawn((
        ChildOf(pivot),
        HeldItemSprite,
        Sprite::default(),
        Anchor::BOTTOM_LEFT,
        Visibility::Hidden,
        ColliderDisabled,
    ));

    RemotePlayerEntry { root, body, attack_seq: 0 }
}

/// Rein visuelles Gegner-Abbild (Health-Bar inklusive, damit das
/// bestehende `draw_enemy_health_bar`-System unverändert funktioniert).
fn spawn_remote_enemy_client(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    id: u64,
    pos: Vec2,
    anim: HumanoidState,
    health: f32,
    max_health: f32,
) -> Entity {
    let clips = Player::get_clips(asset_server, layouts);
    let clip = clips.clips.get(&Anim::Humanoid(anim)).cloned().unwrap();

    let entity = commands.spawn((
        RemoteEnemy,
        NetTarget(pos),
        Enemy::new(),
        Health { health_points: health, max_health },
        Transform::from_xyz(pos.x, pos.y, ZLayer::Enemy.z()),
        Visibility::Visible,
        Anim::Humanoid(anim),
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: clip.image.clone(),
            texture_atlas: Some(TextureAtlas { layout: clip.layout.clone(), index: clip.first }),
            ..default()
        },
        clips,
        AnimationTimer::new(),
    )).id();

    commands.spawn((
        ChildOf(entity),
        Sprite { custom_size: Some(Vec2::new(35.0, 2.0)), color: Color::srgb(1.0, 0.0, 0.0), ..default() },
        Transform::from_xyz(0.0, (PLAYER_HEIGHT / 2.0) + 2.0, ZLayer::EnemyHealthBarForeground.z()),
        EnemyHealthBar,
    ));
    commands.spawn((
        ChildOf(entity),
        Sprite { custom_size: Some(Vec2::new(35.0, 2.0)), color: Color::srgb(0.5, 0.1, 0.1), ..default() },
        Transform::from_xyz(0.0, (PLAYER_HEIGHT / 2.0) + 2.0, ZLayer::EnemyHealthBarBackground.z()),
        EnemyHealthBarBackground,
    ));
    entity
}