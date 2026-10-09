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
use crate::ingame::inventory_meta::*;
use crate::ingame::player::*;
use crate::ingame::ui::*;
use crate::ingame::inventory::*;

pub fn ingame_plugin(app: &mut App) {
    app
        .add_systems(
            OnEnter(GameState::InGame),
            (startup.run_if(not(any_with_component::<Player>)),
            spawn_ui,
            spawn_inventory,
            )

        )
        .add_systems(
            OnExit(GameState::InGame),
            cleanup_ui,
        )
        .add_systems(OnEnter(GameState::Welcome), cleanup_game)
        .add_systems(
            FixedUpdate,
            (controls, set_hotkeys, load_map).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (
                movement,
                update_ui,
                update_inventory_items,
                update_inventory,
                mov_cam,
            ).run_if(in_state(GameState::InGame)),
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
        GameEntity,
        Player {
            inventory: Inventory::default(),
            health: 100.0,
            max_health: 100.0,
            armor: 0.0,
            speed: 1.0,
        },
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

fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
    mut regions: ResMut<RegionPool>,
    mut chunk_pool: ResMut<ChunkPool>,
    mut cams: Query<(&mut Transform, &mut Velocity), With<Camera2d>>,
) {
    for e in &entities {
        commands.entity(e).despawn();
    }
    regions.regions.clear();
    chunk_pool.chunks.clear();

    // Kamera zurücksetzen
    for (mut t, mut v) in &mut cams {
        t.translation = Vec3::new(0.0, 0.0, 1.0);
        v.x = 0.0;
        v.y = 0.0;
    }
}

//
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
    mut next_state: ResMut<NextState<GameState>>,
) {
        velocity.x = 0.0;
        velocity.y = 0.0;
        cam_vel.x = 0.0;
        cam_vel.y = 0.0;

        let cam_pos = cam_transform.translation;
        let mov_speed = s.mov_speed as f32;
        if keys.pressed(KeyCode::KeyW) {
            velocity.y = mov_speed;
            if transform.translation.y + PLAYER_HEIGHT >= cam_pos.y + s.player_walk_border_height {
                cam_vel.y = mov_speed;
            }
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.y = -mov_speed;
            if transform.translation.y <= cam_pos.y - s.player_walk_border_height {
                cam_vel.y = -mov_speed;
            }
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.x = -mov_speed;
            if transform.translation.x <= cam_pos.x - s.player_walk_border_width {
                cam_vel.x = -mov_speed;
            }
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity.x = mov_speed;
            if transform.translation.x + PLAYER_WIDTH >= cam_pos.x + s.player_walk_border_width {
                cam_vel.x = mov_speed;
            }
        }
    if keys.pressed(KeyCode::Escape) {
        next_state.set(GameState::Settings);
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
