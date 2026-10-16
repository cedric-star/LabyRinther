use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;

pub fn is_wall(tx: i32, ty: i32, pool: &ChunkPool, chunks: &Query<&Chunk>) -> bool {
    let cs = CHUNK_SIZE as i32;
    let key = Cords::new(tx.div_euclid(cs), ty.div_euclid(cs));

    // nicht geladener Chunk gilt als Wand
    let Some(&entity) = pool.chunks.get(&key) else { return true; };
    let Ok(chunk) = chunks.get(entity) else { return true; };

    chunk.tiles[tx.rem_euclid(cs) as usize][ty.rem_euclid(cs) as usize] == Tile::Wall
}

pub fn collides(x: f32, y: f32, pool: &ChunkPool, chunks: &Query<&Chunk>) -> bool {
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