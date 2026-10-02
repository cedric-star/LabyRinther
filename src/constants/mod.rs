pub const CAMERA_WIDTH: f32 = 480.0 * 25.0;
pub const CAMERA_HEIGHT: f32 = 270.0 * 25.0;
pub const MAX_WIDTH: f32 = CAMERA_WIDTH / 2.0;
pub const MAX_HEIGHT: f32 = CAMERA_HEIGHT / 2.0;

pub const PLAYER_WALK_BORDER_WIDTH: f32 = MAX_WIDTH / 3.0 * 2.0;
pub const PLAYER_WALK_BORDER_HEIGHT: f32 = MAX_HEIGHT / 3.0 * 2.0;

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

pub const CHUNKS_LOADING: i32 = 10;

pub const MOV_SPEED: f32 = 4.0 * 40.0;

pub const MAX_REGION_SIDE_CONNECTIONS: u64 = 6;
pub const MIN_REGION_SIDE_CONNECTIONS: u64 = 4;

pub const MIN_ROOM_NUM: usize = 5;
pub const MAX_ROOM_NUM: usize = 10;

pub const TILE_SIZE: f32 = 16.0;
pub const CHUNK_SIZE: usize = 8;
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_SIZE as f32 * TILE_SIZE;
pub const REGION_SIZE: i32 = 32; //in chunks
pub const REGION_TILE_SIZE: i32 = REGION_SIZE * CHUNK_SIZE as i32;
pub const REGION_WORLD_SIZE: f32 = REGION_SIZE as f32 * CHUNK_WORLD_SIZE; //absolut, in pixeln

// Zellraster
pub const CELL_FLOOR: i32 = 3; // Gangbreite in Tiles
pub const CELL_PITCH: i32 = 4; // Gang + Wand (dickere Wände: 5)
pub const GRID_SIZE: usize = ((REGION_TILE_SIZE - 1) / CELL_PITCH) as usize;
pub const LOOP_PERCENT: u64 = 4; // Chance in %, zusätzliche Wände zu öffnen
