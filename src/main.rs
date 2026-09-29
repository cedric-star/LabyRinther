mod world;
mod components;
mod resources;
mod constants;

use bevy::prelude::*;
use bevy::sprite::*;
use bevy::camera::*;
use bevy::window::*;
use world::world_generator::*;
use world::room::*;
use world::region::*;
use components::*;
use resources::*;
use constants::*;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut regions: ResMut<RegionPool>,
    seed: Res<WorldSeed>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: CAMERA_WIDTH,
                max_height: CAMERA_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Velocity {x: 0.0, y: 0.0 },
        Anchor::BOTTOM_LEFT,
    ));

    commands.spawn((
        Player {id: 0, x: 0.0, y: 0.0},
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: asset_server.load("character_simple.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Velocity { x: 0.0, y: 0.0 },
        Anchor::BOTTOM_LEFT,
    ));

    let region_entity = commands.spawn(generate_region(seed.seed, 0, 0)).id();

    regions.regions.insert((0, 0), region_entity);
}

fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, (With<Player>, Without<Camera2d>)>,
    transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut cam_vel: Single<&mut Velocity, (With<Camera2d>, Without<Player>)>,
    cam_transform: Single<&Transform, (With<Camera2d>, Without<Player>)>
) {
        velocity.x = 0.0;
        velocity.y = 0.0;
        cam_vel.x = 0.0;
        cam_vel.y = 0.0;

        let cam_pos = cam_transform.translation;

        if keys.pressed(KeyCode::KeyW) {
            velocity.y = MOV_SPEED;
            if transform.translation.y + PLAYER_HEIGHT >= cam_pos.y + PLAYER_WALK_BORDER_HEIGHT {
                cam_vel.y = MOV_SPEED;
            }
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.y = -MOV_SPEED;
            if transform.translation.y <= cam_pos.y - PLAYER_WALK_BORDER_HEIGHT {
                cam_vel.y = -MOV_SPEED;
            }
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.x = -MOV_SPEED;
            if transform.translation.x <= cam_pos.x - PLAYER_WALK_BORDER_WIDTH {
                cam_vel.x = -MOV_SPEED;
            }
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity.x = MOV_SPEED;
            if transform.translation.x + PLAYER_WIDTH >= cam_pos.x + PLAYER_WALK_BORDER_WIDTH {
                cam_vel.x = MOV_SPEED;
            }
        }

}

fn movement(
    mut query_player: Query<(&mut Transform, &Velocity), With<Player>>
)
     {

    for (mut transform, velocity) in &mut query_player {

        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }

}

fn mov_cam(mut query_cam: Query<(&mut Transform, &Velocity), With<Camera2d>>) {

    for (mut transform, velocity) in &mut query_cam {

        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }

}

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<ChunkPool>()
        .init_resource::<RegionPool>()
        .insert_resource(WorldSeed{ seed: 3 })
        .add_systems(Startup, startup)
        .add_systems(FixedUpdate, (controls, load_map))
        .add_systems(Update, (movement, mov_cam))
        .run();
}
