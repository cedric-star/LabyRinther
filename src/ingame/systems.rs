use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::constants::*;
use crate::world::region::*;
use crate::ingame::player::structs::*;
use crate::resources::*;
use std::time::Instant;
use crate::ingame::entity::structs::*;

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut regions: ResMut<RegionPool>,
    seed: Res<WorldSeed>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let start_t = Instant::now();
    commands.spawn(Player::new_entity(&asset_server, &mut texture_atlas_layouts));

    let region_entity = commands.spawn(generate_region(seed.seed, Cords::new(0, 0))).id();

    regions.regions.insert(Cords::new(0, 0), region_entity);

    let end_t = start_t.elapsed().as_millis();
    println!("startup_time: {}ms", end_t);
}

pub fn cleanup_game(
    mut commands: Commands,
    entities: Query<Entity, With<GameEntity>>,
    mut regions: ResMut<RegionPool>,
    mut chunk_pool: ResMut<ChunkPool>,
    mut cams: Query<(&mut Transform, &mut Velocity), With<Camera2d>>,
) {
    println!("cleanup game...");
    for e in &entities {
        commands.entity(e).despawn();
    }
    regions.regions.clear();
    chunk_pool.chunks.clear();

    // Kamera zurücksetzen
    for (mut t, mut v) in &mut cams {
        t.translation = Vec3::new(0.0, 0.0, 1.0);
        v.x = 0.0;
        v.y = 0.0;
    }
}

pub fn mov_cam(mut query_cam: Query<(&mut Transform, &Velocity), With<Camera2d>>) {

    for (mut transform, velocity) in &mut query_cam {

        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }

}