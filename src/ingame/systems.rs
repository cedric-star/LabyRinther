use bevy::prelude::*;
use bevy::app::*;
use bevy::sprite::*;
use crate::components::*;
use crate::constants::*;
use crate::world::world_generator::*;
use crate::world::region::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;
use crate::ingame::player::ui::*;
use crate::ingame::player::util::*;
use crate::ingame::player::systems::*;
use crate::ingame::player::structs::*;
use crate::settings::settings_screen::*;
use crate::ingame::status_bars::*;
use crate::ingame::recipes::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::systems::*;
use crate::ingame::inventory::ui::*;
use crate::ingame::recipes::structs::*;
use crate::ingame::recipes::systems::*;
use crate::ingame::recipes::ui::*;
use crate::ingame::structs::*;
use crate::resources::*;
use bevy::time::common_conditions::on_timer;
use std::time::{Instant, Duration};


pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut regions: ResMut<RegionPool>,
    seed: Res<WorldSeed>,
) {
    let start_t = Instant::now();
    commands.spawn((
        GameEntity,
        Player {
            inventory: Inventory::default(),
            health: 100.0,
            max_health: 100.0,
            mana: 0.0,
            max_mana: 100.0,
            armor: 0.0,
            speed: 1.0,
        },
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: asset_server.load("character_simple.png"),
            ..default()
        },
        Transform::from_xyz(16.0, 16.0, 1.0),
        Velocity { x: 0.0, y: 0.0 },
        Anchor::BOTTOM_LEFT,
    ));

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