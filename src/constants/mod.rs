use bevy::math::*;

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

pub const MAX_REGION_SIDE_CONNECTIONS: u64 = 6;
pub const MIN_REGION_SIDE_CONNECTIONS: u64 = 4;

pub const MIN_ROOM_NUM: usize = 5;
pub const MAX_ROOM_NUM: usize = 10;

pub const TILE_SIZE: i32 = 16; //world px
pub const CHUNK_SIZE: i32 = 8; //tiles
pub const U_CHUNK_SIZE: usize = CHUNK_SIZE as usize;
pub const CHUNK_WORLD_SIZE: i32 = CHUNK_SIZE * TILE_SIZE; //chunk größe in pixeln
pub const REGION_SIZE: i32 = 32; //in chunks
pub const REGION_TILE_SIZE: i32 = REGION_SIZE * CHUNK_SIZE;
pub const REGION_WORLD_SIZE: i32 = REGION_SIZE * CHUNK_WORLD_SIZE; //absolut, in pixeln

//größen als ivec:
pub const TILE_SIZE_VEC2: IVec2 = IVec2::splat(TILE_SIZE);
pub const CHUNK_WORLD_SIZE_VEC2: IVec2 = IVec2::splat(CHUNK_WORLD_SIZE);
pub const REGION_WORLD_SIZE_VEC2: IVec2 = IVec2::splat(REGION_WORLD_SIZE);
pub const CHUNK_SIZE_VEC2: IVec2 = IVec2::splat(CHUNK_SIZE);
pub const REGION_TILE_SIZE_VEC2: IVec2 = IVec2::splat(REGION_TILE_SIZE);
pub const REGION_SIZE_VEC2: IVec2 = IVec2::splat(REGION_SIZE);

// Zellraster
pub const CELL_FLOOR: i32 = 3; // Gangbreite in Tiles
pub const CELL_PITCH: i32 = 4; // Gang + Wand (dickere Wände: 5)
pub const GRID_SIZE: usize = ((REGION_TILE_SIZE - 1) / CELL_PITCH) as usize;
pub const LOOP_PERCENT: u64 = 4; // Chance in %, zusätzliche Wände zu öffnen

pub const DRAW_HITBOX: bool = false;