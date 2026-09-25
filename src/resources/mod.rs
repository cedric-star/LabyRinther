use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct WorldSeed{ pub seed: u64, }

#[derive(Resource, Default)]
pub struct ChunkPool {
    pub chunks: HashMap<(i32, i32), Entity>, }

#[derive(Resource)]
struct Players {
    players: Vec<Entity>,
}
