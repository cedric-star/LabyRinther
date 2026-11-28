// Join-Bildschirm: Adresse eingeben (IP:Port) und verbinden.
// Bewusst ohne extra Text-Input-Widget: eine kleine eigene Tastatur-Abfrage
// reicht für IP:Port und hat keine Abhängigkeits-Risiken.

use bevy::prelude::*;
use bevy::ui::*;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc;
use std::time::Duration;

use crate::components::*;
use crate::net::client::NetClient;
use crate::net::protocol::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;

#[derive(Resource, Default)]
pub struct JoinInput(pub String);

#[derive(Resource, Default)]
pub struct JoinStatus(pub String);

#[derive(Resource, Default)]
pub struct JoinAttempt(pub std::sync::Mutex<Option<mpsc::Receiver<JoinOutcome>>>);

pub enum JoinOutcome {
    Connected(TcpStream),
    Failed(String),
}

#[derive(Component)]
pub struct JoinConnectButton;

#[derive(Component)]
pub struct JoinBackButton;

#[derive(Component)]
pub struct JoinAddressText;

#[derive(Component)]
pub struct JoinStatusText;

pub fn join_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut input: ResMut<JoinInput>,
    mut status: ResMut<JoinStatus>,
    attempt: ResMut<JoinAttempt>,
) {
    input.0 = format!("127.0.0.1:{DEFAULT_PORT}");
    status.0 = String::new();
    *attempt.0.lock().unwrap() = None;

    let btn_images = ButtonImages::load_btn(&asset_server);

    commands.spawn((
        DespawnOnExit(GameState::Join),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(10),
            ..default()
        },
        BackgroundColor(BG_COL),
        children![
            (
                Text::new("Join Game"),
                TextFont { font_size: FontSize::Px(40.0), ..default() },
                TextColor(TXT_COL),
            ),
            (
                Node {
                    width: px(520),
                    height: px(65),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    JoinAddressText,
                    Text::new(""),
                    TextFont { font_size: FontSize::Px(33.0), ..default() },
                    TextColor(TXT_COL),
                )],
            ),
            (
                Text::new("Adresse als IP:Port - ':' gibt es über die ';' Taste"),
                TextFont { font_size: FontSize::Px(20.0), ..default() },
                TextColor(TXT_COL),
            ),
            my_button(JoinConnectButton, "Connect", 100.0, &btn_images),
            my_button(JoinBackButton, "Back", 65.0, &btn_images),
            (
                JoinStatusText,
                Text::new(""),
                TextFont { font_size: FontSize::Px(24.0), ..default() },
                TextColor(TXT_COL),
            ),
        ],
    ));
}

/// Tastatureingabe für das Adressfeld.
pub fn join_keyboard(keys: Res<ButtonInput<KeyCode>>, mut input: ResMut<JoinInput>) {
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::Backspace => { input.0.pop(); }
            KeyCode::Space => input.0.push(' '),
            k => {
                if let Some(c) = key_to_char(*k) {
                    if input.0.len() < 48 { input.0.push(c); }
                }
            }
        }
    }
}

fn key_to_char(key: KeyCode) -> Option<char> {
    use KeyCode::*;
    Some(match key {
        KeyA => 'a', KeyB => 'b', KeyC => 'c', KeyD => 'd', KeyE => 'e', KeyF => 'f',
        KeyG => 'g', KeyH => 'h', KeyI => 'i', KeyJ => 'j', KeyK => 'k', KeyL => 'l',
        KeyM => 'm', KeyN => 'n', KeyO => 'o', KeyP => 'p', KeyQ => 'q', KeyR => 'r',
        KeyS => 's', KeyT => 't', KeyU => 'u', KeyV => 'v', KeyW => 'w', KeyX => 'x',
        KeyY => 'y', KeyZ => 'z',
        Digit0 | Numpad0 => '0', Digit1 | Numpad1 => '1', Digit2 | Numpad2 => '2',
        Digit3 | Numpad3 => '3', Digit4 | Numpad4 => '4', Digit5 | Numpad5 => '5',
        Digit6 | Numpad6 => '6', Digit7 | Numpad7 => '7', Digit8 | Numpad8 => '8',
        Digit9 | Numpad9 => '9',
        Period => '.',
        Semicolon => ':', // ':' gibt es als eigene KeyCode-Taste nicht
        Minus => '-',
        _ => return None,
    })
}

/// Connect-Klick: Verbindungsversuch in einem Hintergrund-Thread starten.
pub fn join_connect_button(
    query: Query<Has<Pressed>, With<JoinConnectButton>>,
    input: Res<JoinInput>,
    mut attempt: ResMut<JoinAttempt>,
    mut status: ResMut<JoinStatus>,
) {
    for pressed in &query {
        if !pressed { continue; }
        let mut guard = attempt.0.lock().unwrap();
        if guard.is_some() { continue; } // Versuch läuft schon

        let addr = input.0.trim().to_string();
        if addr.is_empty() {
            status.0 = "Bitte eine Adresse eingeben".to_string();
            continue;
        }
        let (tx, rx) = mpsc::channel();
        *guard = Some(rx);
        drop(guard);
        status.0 = "Verbinde...".to_string();

        std::thread::spawn(move || {
            let outcome = match connect_to(&addr) {
                Ok(stream) => JoinOutcome::Connected(stream),
                Err(e) => JoinOutcome::Failed(e),
            };
            let _ = tx.send(outcome);
        });
    }
}

fn connect_to(addr: &str) -> Result<TcpStream, String> {
    let mut addrs = addr.to_socket_addrs().map_err(|e| format!("Ungültige Adresse: {e}"))?;
    let Some(socket_addr) = addrs.next() else {
        return Err("Adresse nicht auflösbar".to_string());
    };
    TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)).map_err(|e| format!("{e}"))
}

/// Ergebnis des Verbindungsversuchs abholen.
pub fn join_poll_attempt(
    mut commands: Commands,
    attempt: ResMut<JoinAttempt>,
    mut status: ResMut<JoinStatus>,
) {
    let Some(rx) = attempt.0.lock().unwrap().take() else { return };
    match rx.try_recv() {
        Ok(JoinOutcome::Connected(stream)) => {
            match NetClient::connect(stream) {
                Ok(mut client) => {
                    client.send(&ClientMessage::JoinRequest { protocol: PROTOCOL_VERSION });
                    status.0 = "Verbunden! Warte auf Welt...".to_string();
                    commands.insert_resource(client);
                }
                Err(e) => status.0 = format!("Verbindung fehlgeschlagen: {e}"),
            }
        }
        Ok(JoinOutcome::Failed(e)) => {
            status.0 = format!("Fehlgeschlagen: {e}");
        }
        Err(mpsc::TryRecvError::Empty) => {
            // noch nicht fertig -> zurücklegen
            *attempt.0.lock().unwrap() = Some(rx);
        }
        Err(mpsc::TryRecvError::Disconnected) => {
            status.0 = "Verbindungsversuch abgebrochen".to_string();
        }
    }
}

pub fn join_back_button(
    query: Query<Has<Pressed>, With<JoinBackButton>>,
    mut attempt: ResMut<JoinAttempt>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for pressed in &query {
        if !pressed { continue; }
        *attempt.0.lock().unwrap() = None;
        next_state.set(GameState::Welcome);
    }
}

pub fn join_update_texts(
    input: Res<JoinInput>,
    status: Res<JoinStatus>,
    mut addr: Query<&mut Text, (With<JoinAddressText>, Without<JoinStatusText>)>,
    mut stat: Query<&mut Text, (With<JoinStatusText>, Without<JoinAddressText>)>,
) {
    let addr_text = format!("{}_", input.0);
    for mut t in &mut addr {
        if t.0 != addr_text { t.0 = addr_text.clone(); }
    }
    for mut t in &mut stat {
        if t.0 != status.0 { t.0 = status.0.clone(); }
    }
}