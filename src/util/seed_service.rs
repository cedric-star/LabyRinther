use std::hash::{DefaultHasher, Hash, Hasher};
use crate::components::*;

pub fn calc_chunk_seed(seed: u64, x: i32, y: i32) -> u64 {
    let mut s = DefaultHasher::new();
    (x, y, seed).hash(&mut s);
    s.finish()
}

pub fn calc_tile_seed(seed: u64, x: i32, y: i32) -> u64 {
    calc_chunk_seed(seed, x, y)
}

pub fn calc_region_seed(seed: u64, x: i32, y: i32) -> u64 {
    calc_chunk_seed(seed, x, y)
}

pub fn calc_connection_seed(seed: u64, x: i32, y: i32) -> u64 {
    calc_chunk_seed(seed, x, y)
}

pub fn calc_room_seed(seed: u64, num: i32, index: i32) -> u64 {
    calc_chunk_seed(seed, num, index)
}

pub fn calc_any_hash(seed:u64, num: i32) -> u64 {
    let mut s = DefaultHasher::new();
    (seed, num).hash(&mut s);
    s.finish()
}

pub fn calc_any_hash_2(seed:u64, x: i32, y: i32) -> u64 {
    let mut s = DefaultHasher::new();
    (seed, x, y).hash(&mut s);
    s.finish()
}
