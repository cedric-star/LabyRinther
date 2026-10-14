mod world;
mod components;
mod resources;
mod constants;
mod util;
mod welcome;
mod ingame;
mod settings;

use bevy::prelude::*;
use bevy::sprite::*;
use bevy::camera::*;
use bevy::window::*;
use world::world_generator::*;
use world::region::*;
use components::*;
use resources::*;
use constants::*;
use std::time::*;
use welcome::welcome_screen::*;
use ingame::ingame_screen::*;
use settings::settings_screen::*;
use ingame::player::*;
use ingame::inventory_meta::*;
use serde_json::{Result, Value};
use std::fs;
use ingame::items::*;
use ingame::recipes::*;

fn setup_cam(
    mut commands: Commands,
    s: Res<GameSettings>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: s.camera_width,
                max_height: s.camera_height,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Velocity {x: 0.0, y: 0.0 },
        Anchor::BOTTOM_LEFT,
    ));
}

fn update_cam(
    s: Res<GameSettings>,
    mut cameras: Query<&mut Projection, With<Camera2d>>,
) {
    if !s.is_changed() {
        return;
    }
    for mut projection in &mut cameras {
        if let Projection::Orthographic(ortho) = &mut *projection {
            ortho.scaling_mode = ScalingMode::AutoMax {
                max_width: s.camera_width,
                max_height: s.camera_height,
            }
        }
    }
}

fn track_previous_state(
    mut reader: MessageReader<StateTransitionEvent<GameState>>,
    mut previous: ResMut<LastState>,
) {
    for event in reader.read() {
        previous.0 = event.exited.clone();
    }
}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .init_resource::<Items>()
        .init_resource::<Recipes>()
        .init_resource::<GameSettings>()
        .init_resource::<LastState>()
        .init_resource::<ChunkPool>()
        .init_resource::<RegionPool>()
        .insert_resource(WorldSeed{ seed: 3 })
        .add_systems(Startup, setup_cam)
        .add_systems(StateTransition, track_previous_state.before(EnterSchedules::<GameState>::default()),)
        .add_systems(Update, (update_cam, track_previous_state))
        .add_plugins((welcome_plugin, ingame_plugin, settings_plugin))
        .run();
}
