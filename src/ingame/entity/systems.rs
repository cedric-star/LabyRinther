use bevy::prelude::*;
use crate::ingame::player::structs::*;
use crate::ingame::entity::util::*;
use crate::components::*;
use crate::resources::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::attack::structs::*;
use crate::ingame::animation::*;
use crate::constants::*;


pub fn draw_enemy_health_bar(
    enemies: Query<&Health, With<Enemy>>,
    mut bars: Query<(&ChildOf, &mut Sprite, &mut Transform), With<EnemyHealthBar>>,
) {
    const FULL: f32 = 35.0;
    for (child_of, mut sprite, mut tf) in &mut bars {
        let Ok(health) = enemies.get(child_of.parent()) else { continue };
        let ratio = (health.health_points / health.max_health).clamp(0.0, 1.0);
        sprite.custom_size = Some(Vec2::new(FULL * ratio, 2.0));
        // linksbündig halten: Mitte nach links verschieben
        tf.translation.x = -FULL * (1.0 - ratio) * 0.5;
    }
}

pub fn despawn_enemy_by_health(
    enemies: Query<(&Health, Entity), With<Enemy>>,
    mut commands: Commands,
) {
    for (health, entity) in &enemies {
        if health.health_points <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}


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
    //println!("chunk_num: {:?}", chunk_num);

    //println!("region_cords: {:?}", region.cords);
    //println!("player_cords: {:?}", p_pos);

    commands.spawn(get_enemy_bundle(
        Cords::from_vec3(p_pos), 
        &asset_server, 
        &mut texture_atlas_layouts
    ));


}

pub fn draw_hitbox(
    q: Query<(&Transform, &Hitbox), Or<(With<Attacker>, With<HandPivot>)>>,
    mut gizmos: Gizmos,
) {
    if !DRAW_HITBOX { return; }
    for (transform, hitbox) in &q {
        hitbox.draw(&mut gizmos, *transform);
    }
}