// Multiplayer: Host ("Listen Server") + Clients.
//
// Grundidee:
// - Der Host simuliert die Welt (Gegner, Wellen, Schaden) und spielt selbst mit.
// - Jeder Client simuliert nur seinen EIGENEN Spieler und schickt Position,
//   Animation und Angriffe an den Host.
// - Die Welt ist aus dem Seed deterministisch generiert -> es muss nur der
//   Seed übertragen werden, Chunks/Regionen baut jeder selbst.
// - Entfernte Spieler/Gegner werden als "Geister" dargestellt und an die
//   Netzwerk-Position herangezogen.

pub mod protocol;
pub mod server;
pub mod client;
pub mod join_ui;

use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::GameState;
use crate::net::client::NetClient;
use crate::net::server::NetServer;
use crate::util::ui::buttons::button_visuals;

// ---------------------------------------------------------------------
// Marker & Komponenten für Netzwerk-Geister
// ---------------------------------------------------------------------

/// Abbild eines entfernten Spielers (auf dem Host UND auf anderen Clients).
/// Bewusst NICHT `Player`: so sehen alle bestehenden Systeme (Steuerung,
/// Inventar, UI, Kamera, ...) weiterhin nur den lokalen Spieler.
#[derive(Component)]
pub struct RemotePlayer;

/// Sprite-/Anim-Körper eines RemotePlayers (entspricht PlayerBody).
#[derive(Component)]
pub struct RemotePlayerBody;

/// Gegner-Abbild auf dem Client (rein visuell, keine Physik).
#[derive(Component)]
pub struct RemoteEnemy;

#[derive(Component)]
pub struct NetPlayerId(pub u64);

/// Ziel-Position aus dem Netzwerk. Der Geist wird dorthin gezogen:
/// - Host: über LinearVelocity (Physik-Körper, damit Angriffe treffen)
/// - Client: über Interpolation (rein visuell)
#[derive(Component)]
pub struct NetTarget(pub Vec2);

/// Zählt Angriffs-Starts (nur Host-Seite), damit Empfänger denselben
/// Angriff nicht doppelt abspielen.
#[derive(Component, Default)]
pub struct AttackTracker {
    pub seq: u32,
    pub last_elapsed: Option<f32>,
}

#[derive(Resource, Default)]
pub struct RemotePlayers(pub HashMap<u64, RemotePlayerEntry>);

#[derive(Resource, Default)]
pub struct RemoteEnemies(pub HashMap<u64, Entity>);

#[derive(Clone, Copy)]
pub struct RemotePlayerEntry {
    pub root: Entity,
    pub body: Entity,
    /// letzter abgespielter Angriff (nur auf der Client-Seite genutzt)
    pub attack_seq: u32,
}

// ---------------------------------------------------------------------
// Netzwerk-Takt
// ---------------------------------------------------------------------

#[derive(Resource)]
pub struct ServerSnapshotTimer(pub Timer);
impl Default for ServerSnapshotTimer {
    fn default() -> Self { Self(Timer::from_seconds(0.05, TimerMode::Repeating)) } // 20 Hz
}

#[derive(Resource)]
pub struct ClientSendTimer(pub Timer);
impl Default for ClientSendTimer {
    fn default() -> Self { Self(Timer::from_seconds(0.05, TimerMode::Repeating)) } // 20 Hz
}

/// Ersatz für das ursprüngliche `on_timer(200ms)` von `spawn_enemy_near_player`,
/// kombiniert mit der Multiplayer-Bedingung (nur Host/Singleplayer).
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);
impl Default for EnemySpawnTimer {
    fn default() -> Self { Self(Timer::from_seconds(0.2, TimerMode::Repeating)) }
}

// ---------------------------------------------------------------------
// Run conditions
// ---------------------------------------------------------------------

/// Läuft diese Instanz als Host?
pub fn has_server(server: Option<Res<NetServer>>) -> bool { server.is_some() }

/// Ist diese Instanz mit einem Host verbunden?
pub fn has_client(client: Option<Res<NetClient>>) -> bool { client.is_some() }

/// Hat diese Instanz die Simulation? (Singleplayer + Host: ja, Client: nein)
pub fn has_authority(client: Option<Res<NetClient>>) -> bool { client.is_none() }

pub fn hosting_in_game(server: Option<Res<NetServer>>, state: Res<State<GameState>>) -> bool {
    server.is_some() && *state.get() == GameState::InGame
}

pub fn connected_in_game(client: Option<Res<NetClient>>, state: Res<State<GameState>>) -> bool {
    client.is_some() && *state.get() == GameState::InGame
}

pub fn authority_in_game(client: Option<Res<NetClient>>, state: Res<State<GameState>>) -> bool {
    client.is_none() && *state.get() == GameState::InGame
}

/// System: tickt den Timer (darf mutieren)
pub fn tick_enemy_spawn_timer(
    client: Option<Res<NetClient>>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
) {
    if client.is_some() { return; }
    timer.0.tick(time.delta());
}

/// Run Condition: liest nur, kein Seiteneffekt
pub fn enemy_spawn_ready(
    client: Option<Res<NetClient>>,
    timer: Res<EnemySpawnTimer>,
) -> bool {
    client.is_none() && timer.0.just_finished()
}


// ---------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------

pub fn net_plugin(app: &mut App) {
    use crate::net::server as srv;
    use crate::net::client as cli;
    use crate::net::join_ui as jui;

    app
        .init_resource::<RemotePlayers>()
        .init_resource::<RemoteEnemies>()
        .init_resource::<ServerSnapshotTimer>()
        .init_resource::<ClientSendTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<jui::JoinInput>()
        .init_resource::<jui::JoinStatus>()
        .init_resource::<jui::JoinAttempt>()

        // ---- Join-Bildschirm ----
        .add_systems(OnEnter(GameState::Join), jui::join_setup)
        .add_systems(Update, (
            jui::join_keyboard,
            jui::join_connect_button,
            jui::join_poll_attempt,
            jui::join_back_button,
            jui::join_update_texts,
            button_visuals,
        ).run_if(in_state(GameState::Join)))

        // ---- Host-Seite ----
        .add_systems(Update, srv::server_accept_new_connections.run_if(has_server))
        .add_systems(Update, srv::server_handle_messages.run_if(has_server))
        .add_systems(Update, srv::server_send_snapshots.run_if(has_server))
        .add_systems(Update, srv::track_attack_seqs.run_if(authority_in_game))
        .add_systems(FixedUpdate, srv::drive_remote_player_bodies.run_if(authority_in_game))
        .add_systems(OnEnter(GameState::InGame), srv::spawn_server_status_ui.run_if(has_server))
        .add_systems(Update, (
            srv::server_toggle_joins,
            srv::update_server_status_text,
        ).run_if(hosting_in_game))

        // ---- Client-Seite ----
        .add_systems(Update, cli::client_handle_messages.run_if(has_client))
        .add_systems(Update, cli::client_send_state.run_if(connected_in_game))
        .add_systems(Update, cli::interpolate_remotes.run_if(has_client))

        // ---- Aufräumen bei Rückkehr ins Hauptmenü ----
        .add_systems(OnEnter(GameState::Welcome), net_cleanup);
}

/// Trennt alle Verbindungen und entfernt alle Netzwerk-Geister.
pub fn net_cleanup(
    mut commands: Commands,
    mut server: Option<ResMut<NetServer>>,
    mut client: Option<ResMut<NetClient>>,
    ghosts: Query<Entity, Or<(With<RemotePlayer>, With<RemoteEnemy>)>>,
    mut remote_players: ResMut<RemotePlayers>,
    mut remote_enemies: ResMut<RemoteEnemies>,
) {
    for e in &ghosts {
        commands.entity(e).despawn();
    }
    remote_players.0.clear();
    remote_enemies.0.clear();

    if let Some(server) = server.as_mut() {
        for conn in server.clients.values() {
            let _ = conn.stream.shutdown(std::net::Shutdown::Both);
        }
        commands.remove_resource::<NetServer>();
    }
    if let Some(client) = client.as_mut() {
        let _ = client.writer.shutdown(std::net::Shutdown::Both);
        commands.remove_resource::<NetClient>();
    }
}