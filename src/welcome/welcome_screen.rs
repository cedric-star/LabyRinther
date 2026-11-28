use bevy::prelude::*;
use bevy::ui::*;
use crate::components::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;
use crate::net::protocol::DEFAULT_PORT;   
use crate::net::server::NetServer; 

pub fn welcome_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Welcome), welcome_setup)
        .add_systems(Update, (
            welcome_button,
            quit_button,
            settings_button,
            button_visuals,
            host_button,
            join_button,
        ).run_if(in_state(GameState::Welcome)));
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct HostButton; 

#[derive(Component)]
struct JoinButton; 

fn welcome_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let btn_images = ButtonImages::load_btn(&asset_server);
    let radio_images = ButtonImages::load_btn_radio(&asset_server);

    commands.spawn((
        DespawnOnExit(GameState::Welcome),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(BG_COL),
        children![
            my_button(StartButton, "Start", 100.0, &btn_images),
            my_button(HostButton, "Host Game", 100.0, &btn_images),
            my_button(JoinButton, "Join Game", 100.0, &btn_images),
            my_button(SettingsButton, "Settings", 100.0, &btn_images),
            my_button(QuitButton, "Quit", 100.0, &btn_images),
        ],
    ));
}
fn welcome_button(
    query: Query<
            Has<Pressed>, With<StartButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for pressed in &query {
        if pressed { next_state.set(GameState::InGame); }    
    }
}

fn host_button(
    query: Query<Has<Pressed>, With<HostButton>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for pressed in &query {
        if !pressed { continue; }
        match NetServer::start(DEFAULT_PORT) {
            Ok(server) => {
                println!("Server läuft: {}:{} (Beitritte: F10)", server.lan_ip, DEFAULT_PORT);
                commands.insert_resource(server);
            }
            Err(e) => println!("Server konnte nicht gestartet werden: {e}"),
        }
        next_state.set(GameState::InGame);
    }
}

fn join_button(
    query: Query<Has<Pressed>, With<JoinButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for pressed in &query {
        if pressed { next_state.set(GameState::Join); }
    }
}

fn quit_button(
    query: Query<Has<Pressed>, With<QuitButton>>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for pressed in &query {
        if pressed { app_exit_writer.write(AppExit::Success); }
    }
}

fn settings_button(
    query: Query<Has<Pressed>, With<SettingsButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for pressed in &query {
        if pressed { next_state.set(GameState::Settings); }
    }
}