use bevy::prelude::*;
use crate::ingame::player::structs::*;
use crate::ingame::entity::util::*;
use crate::components::*;
use crate::resources::*;

pub fn spawn_enemy_near_player(
    player: Single<&Transform, With<Player>>,
    region_pool: Res<RegionPool>,
    regions: Query<&Region>,
    chunks: Query<&Chunk>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //player
    let p_pos = player.translation;

    //region
    let r_cords = Cords::new(p_pos.x as i32, p_pos.y as i32).world_to_region();
    let Some(&region_entity) = region_pool.regions.get(&r_cords) else { return; };
    let Ok(region) = regions.get(region_entity) else { return; };

    //chunks loaded
    let mut chunk_num = 0;
    for chunk in &chunks { chunk_num += 1; }
    println!("chunk_num: {:?}", chunk_num);

    println!("region_cords: {:?}", region.cords);
    println!("player_cords: {:?}", p_pos);

    commands.spawn(get_enemy_bundle(
        Cords::from_vec3(p_pos), 
        &asset_server, 
        &mut texture_atlas_layouts
    ));


}