pub const CAMERA_WIDTH: f32 = 480.0 * 3.0;
pub const CAMERA_HEIGHT: f32 = 270.0 * 3.0;
pub const MAX_WIDTH: f32 = CAMERA_WIDTH / 2.0;
pub const MAX_HEIGHT: f32 = CAMERA_HEIGHT / 2.0;

pub const PLAYER_WALK_BORDER_WIDTH: f32 = MAX_WIDTH / 3.0 * 2.0;
pub const PLAYER_WALK_BORDER_HEIGHT: f32 = MAX_HEIGHT / 3.0 * 2.0;

pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

pub const CHUNKS_LOADING: i32 = 2;

pub const MOV_SPEED: f32 = 4.0 * 6.0;

pub const TILE_SIZE: f32 = 16.0;
pub const CHUNK_SIZE: usize = 8;
pub const CHUNK_WORLD_SIZE: f32 = CHUNK_SIZE as f32 * TILE_SIZE;
pub const REGION_SIZE: i32 = 32; //in chunks
pub const REGION_TILE_SIZE: i32 = REGION_SIZE * CHUNK_SIZE as i32;
pub const REGION_WORLD_SIZE: i32 = REGION_SIZE * CHUNK_WORLD_SIZE as i32; //absolut, in pixeln
//
