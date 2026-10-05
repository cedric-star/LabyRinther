use bevy::prelude::*;
use bevy::sprite::*;
use bevy::camera::*;
use bevy::window::*;
use crate::world::world_generator::*;
use crate::world::region::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use std::time::*;
use crate::settings::settings_screen::*;

pub fn ingame_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::InGame), startup)
        .add_systems(
            FixedUpdate,
            (controls, load_map).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (movement, mov_cam).run_if(in_state(GameState::InGame)),
        );
}
fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut regions: ResMut<RegionPool>,
    seed: Res<WorldSeed>,
) {
    let start_t = Instant::now();

    commands.spawn((
        Player {id: 0, x: 0.0, y: 0.0},
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: asset_server.load("character_simple.png"),
            ..default()
        },
        Transform::from_xyz(16.0, 16.0, 1.0),
        Velocity { x: 0.0, y: 0.0 },
        Anchor::BOTTOM_LEFT,
    ));

    let region_entity = commands.spawn(generate_region(seed.seed, Cords::new(0, 0))).id();

    regions.regions.insert(Cords::new(0, 0), region_entity);

    let end_t = start_t.elapsed().as_millis();
    println!("startup_time: {}ms", end_t);
}

//
//
//
//
//
//
//
//

fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, (With<Player>, Without<Camera2d>)>,
    transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut cam_vel: Single<&mut Velocity, (With<Camera2d>, Without<Player>)>,
    cam_transform: Single<&Transform, (With<Camera2d>, Without<Player>)>,
    s: Res<GameSettings>,
) {
        velocity.x = 0.0;
        velocity.y = 0.0;
        cam_vel.x = 0.0;
        cam_vel.y = 0.0;

        let cam_pos = cam_transform.translation;

        if keys.pressed(KeyCode::KeyW) {
            velocity.y = s.MOV_SPEED;
            if transform.translation.y + PLAYER_HEIGHT >= cam_pos.y + PLAYER_WALK_BORDER_HEIGHT {
                cam_vel.y = s.MOV_SPEED;
            }
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.y = -s.MOV_SPEED;
            if transform.translation.y <= cam_pos.y - PLAYER_WALK_BORDER_HEIGHT {
                cam_vel.y = -s.MOV_SPEED;
            }
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.x = -s.MOV_SPEED;
            if transform.translation.x <= cam_pos.x - PLAYER_WALK_BORDER_WIDTH {
                cam_vel.x = -s.MOV_SPEED;
            }
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity.x = s.MOV_SPEED;
            if transform.translation.x + PLAYER_WIDTH >= cam_pos.x + PLAYER_WALK_BORDER_WIDTH {
                cam_vel.x = s.MOV_SPEED;
            }
        }

}

fn is_wall(tx: i32, ty: i32, pool: &ChunkPool, chunks: &Query<&Chunk>) -> bool {
    let cs = CHUNK_SIZE as i32;
    let key = Cords::new(tx.div_euclid(cs), ty.div_euclid(cs));

    // nicht geladener Chunk gilt als Wand
    let Some(&entity) = pool.chunks.get(&key) else { return true; };
    let Ok(chunk) = chunks.get(entity) else { return true; };

    chunk.tiles[tx.rem_euclid(cs) as usize][ty.rem_euclid(cs) as usize] == Tile::Wall
}

fn collides(x: f32, y: f32, pool: &ChunkPool, chunks: &Query<&Chunk>) -> bool {
    let x0 = (x / TILE_SIZE).floor() as i32;
    let x1 = ((x + PLAYER_WIDTH - 0.01) / TILE_SIZE).floor() as i32;
    let y0 = (y / TILE_SIZE).floor() as i32;
    let y1 = ((y + PLAYER_HEIGHT - 0.01) / TILE_SIZE).floor() as i32;

    for tx in x0..=x1 {
        for ty in y0..=y1 {
            if is_wall(tx, ty, pool, chunks) {
                return true;
            }
        }
    }
    false
}

fn movement(
    mut query_player: Query<(&mut Transform, &Velocity), With<Player>>,
    chunk_pool: Res<ChunkPool>,
    chunks: Query<&Chunk>,
) {
    for (mut transform, velocity) in &mut query_player {
        let mut pos = transform.translation;

        // X-Achse
        let nx = pos.x + velocity.x;
        if collides(nx, pos.y, &chunk_pool, &chunks) {
            if velocity.x > 0.0 {
                pos.x = ((nx + PLAYER_WIDTH) / TILE_SIZE).floor() * TILE_SIZE - PLAYER_WIDTH;
            } else if velocity.x < 0.0 {
                pos.x = ((nx / TILE_SIZE).floor() + 1.0) * TILE_SIZE;
            }
        } else {
            pos.x = nx;
        }

        // Y-Achse
        let ny = pos.y + velocity.y;
        if collides(pos.x, ny, &chunk_pool, &chunks) {
            if velocity.y > 0.0 {
                pos.y = ((ny + PLAYER_HEIGHT) / TILE_SIZE).floor() * TILE_SIZE - PLAYER_HEIGHT;
            } else if velocity.y < 0.0 {
                pos.y = ((ny / TILE_SIZE).floor() + 1.0) * TILE_SIZE;
            }
        } else {
            pos.y = ny;
        }

        transform.translation = pos;
    }
}
fn mov_cam(mut query_cam: Query<(&mut Transform, &Velocity), With<Camera2d>>) {

    for (mut transform, velocity) in &mut query_cam {

        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }

}
