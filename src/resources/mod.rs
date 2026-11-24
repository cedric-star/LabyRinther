use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct WorldSeed{ pub seed: u64, }

#[derive(Resource, Default)]
pub struct ChunkPool {
    pub chunks: HashMap<IVec2, Entity>, }

#[derive(Resource, Default)]
pub struct RegionPool {
    pub regions: HashMap<IVec2, Entity>,
}

#[derive(Resource, Default)]
pub struct Enemies {
    pub num: u32,
    pub max: u32,
}

impl Default for Enemies {
    fn default() -> Self {
        Slef {
            num: 0,
            max: 4,
        }
    }
}

#[derive(Resource)]
struct Players {
    players: Vec<Entity>,
}
