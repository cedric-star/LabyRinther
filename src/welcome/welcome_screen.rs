use bevy::prelude::*;
use bevy::app::*;
use crate::components::*;

pub fn welcome_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Welcome), welcome_setup)
        .add_systems(Update, (
            welcome_button,
            quit_button,
            settings_button,
        ).run_if(in_state(GameState::Welcome)));
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct SettingsButton;

fn welcome_setup(mut commands: Commands) {
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
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        children![(
            Button,
            StartButton,
            Node {
                width: px(300),
                height: px(65),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            children![(
                Text::new("Start"),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],
        ),(
            Button,
            SettingsButton,
            Node {
                width: px(300),
                height: px(65),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            children![(
                Text::new("Settings"),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],

        ),(
            Button,
            QuitButton,
            Node {
                width: px(300),
                height: px(65),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(1.0, 0.15,0.15)),
            children![(
                Text::new("Quit"),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],

        )],
    ));
}

fn welcome_button(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            println!("ingame!");
            next_state.set(GameState::InGame);
        }
    }
}

fn quit_button(
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_writer: MessageWriter<AppExit>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            println!("exit!!!");
            app_exit_writer.write(AppExit::Success);
        }
    }
}

fn settings_button(
    query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Settings);
        }
    }
}
