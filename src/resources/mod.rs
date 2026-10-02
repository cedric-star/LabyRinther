use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::*;

#[derive(Resource, Default)]
pub struct WorldSeed{ pub seed: u64, }

#[derive(Resource, Default)]
pub struct ChunkPool {
    pub chunks: HashMap<Cords, Entity>, }

#[derive(Resource, Default)]
pub struct RegionPool {
    pub regions: HashMap<Cords, Entity>,
}

#[derive(Resource)]
struct Players {
    players: Vec<Entity>,
}
