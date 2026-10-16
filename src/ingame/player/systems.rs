use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::util::*;
use crate::ingame::player::structs::*;

pub fn mana_regen(mut player: Query<&mut Player>) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    if player.mana == player.max_mana {
        return;
    }
    let mana_regen_num = 2.0;
    if (player.mana + mana_regen_num) >= player.max_mana {
        player.mana = player.max_mana;
        return;
    }

    player.mana += mana_regen_num;
}

pub fn health_regen(mut player: Query<&mut Player>) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    if player.health == player.max_health {
        return;
    }
    let health_regen_num = 2.0;
    if (player.health + health_regen_num) >= player.max_health {
        player.health = player.max_health;
        return;
    }

    player.health += health_regen_num;
}

pub fn controls(
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

pub fn movement(
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