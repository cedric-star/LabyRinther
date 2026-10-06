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

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .init_resource::<GameSettings>()
        .init_resource::<ChunkPool>()
        .init_resource::<RegionPool>()
        .insert_resource(WorldSeed{ seed: 3 })
        .add_systems(Startup, setup_cam)
        .add_systems(Update, update_cam)
        .add_plugins((welcome_plugin, ingame_plugin, settings_plugin))
        .run();
}
