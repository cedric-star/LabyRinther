use bevy::prelude::*;
use crate::ingame::player::structs::*;
use crate::components::*;
use crate::resources::*;

pub fn spawn_enemy_near_player(
    player: Single<&Transform, With<Player>>,
    region_pool: Res<RegionPool>,
    regions: Query<&Region>,
) {
    let pos = player.translation;
    let r_cords = Cords::new(pos.x as i32, pos.y as i32).world_to_region();
    let Some(&region_entity) = region_pool.regions.get(&r_cords) else { return; };
    let Ok(region) = regions.get(region_entity) else { return; };

    println!("{:?}, {:?}", region.rooms, region.cords);
}