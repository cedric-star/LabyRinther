use bevy::prelude::*;
use bevy::app::*;
use crate::components::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;

pub fn welcome_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Welcome), welcome_setup)
        .add_systems(Update, (
            welcome_button,
            quit_button,
            settings_button,
            button_visuals,
        ).run_if(in_state(GameState::Welcome)));
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct SettingsButton;

fn welcome_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let images = ButtonImages::load(&asset_server);

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
            my_button(StartButton, "Start", 100.0, &images),
            my_button(SettingsButton, "Settings", 100.0, &images),
            my_button(QuitButton, "Quit", 100.0, &images),
        ],
    ));
}
fn welcome_button(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut was_pressed: Local<bool>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Pressed => *was_pressed = true,
            Interaction::Hovered if *was_pressed => {
                *was_pressed = false;
                println!("ingame!");
                next_state.set(GameState::InGame);
            }
            _ => *was_pressed = false,
        }
    }
}

fn quit_button(
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut was_pressed: Local<bool>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Pressed => *was_pressed = true,
            Interaction::Hovered if *was_pressed => {
                *was_pressed = false;
                println!("exit!!!");
                app_exit_writer.write(AppExit::Success);
            }
            _ => *was_pressed = false,
        }
    }
}

fn settings_button(
    query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut was_pressed: Local<bool>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        match *interaction {
            Interaction::Pressed => *was_pressed = true,
            Interaction::Hovered if *was_pressed => {
                *was_pressed = false;
                next_state.set(GameState::Settings);
            }
            _ => *was_pressed = false,
        }
    }
}
