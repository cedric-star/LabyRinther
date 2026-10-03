use bevy::prelude::*;
use crate::components::*;

pub fn welcome_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Welcome), welcome_setup)
        .add_systems(Update, welcome_button.run_if(in_state(GameState::Welcome)));
}

fn welcome_setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Welcome),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        children![(
            Button,
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
        )],
    ));
}

fn welcome_button(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }
}
