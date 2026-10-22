use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::*;
use crate::ingame::inventory::structs::*;
use std::collections::HashMap;

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

//returns armor points from chest, helm, ... 0 if none
pub fn extract_armor_points_from_armor_def(
    def: Option<ItemStack>, 
    items: &HashMap<ItemId, ItemDef>,
) -> f32 {
    if let Some(armor) = def {
        let Some(item_def) = items.get(&armor.id) else { return 0.0; };
        let Some(armor_def) = item_def.armor else { return 0.0; };
        return armor_def.points;
    } else { return 0.0; }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            inventory: Inventory::default(),
            health: 100.0,
            max_health: 100.0,
            health_regen: 1.0,
            mana: 0.0,
            max_mana: 100.0,
            mana_regen: 1.0,
            armor: 0.0,
            speed: 1.0,
        }
    }
}