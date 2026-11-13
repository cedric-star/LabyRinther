
use bevy::prelude::*;
use crate::ingame::player::structs::*;
use crate::settings::settings_screen::*;
use crate::ingame::entity::util::*;
use crate::components::*;
use crate::resources::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::attack::structs::*;
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
    s: Res<GameSettings>,
    player: Single<&Transform, With<Player>>,
    region_pool: Res<RegionPool>,
    regions: Query<&Region>,
    chunks: Query<&Chunk>,
    chunk_pool: Res<ChunkPool>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //player
    let p_pos = player.translation;
    let p_chunk_pos = IVec2::new(p_pos.x as i32, p_pos.y as i32).world_to_chunk();

    //regionp
    let r_cords = IVec2::new(p_pos.x as i32, p_pos.y as i32).world_to_region();
    let Some(&region_entity) = region_pool.regions.get(&r_cords) else { return; };
    let Ok(region) = regions.get(region_entity) else { return; };

    //chunks loaded
    let mut chunk_num = chunks.iter().count();

    //all chunks as vec
    let chunk_vec: Vec<&Chunk> = chunks.iter().collect();

    //filtered, only chunks near player
    let chunks_2_spawn_enemies: Vec<&Chunk> = chunks_spawn_enemy(chunk_vec, &chunk_pool, s.chunks_enemy_spawnable, p_chunk_pos);

    //a world koordinate, where a place is free
    let enemy_spawn_point: Vec3 = find_enemy_spawn_point(p_pos, chunks_2_spawn_enemies);


    commands.spawn(get_enemy_bundle(
        enemy_spawn_point,
        &asset_server, 
        &mut texture_atlas_layouts
    ));
}

//lifetime checker weis nicht, ob rückgabe chunks aus vec<chunks> oder chunkpool stammt -> fehler
//lsg: 1. lifetimeannotation: 'a in funktion erstellt lifetime namens a
//2. 'a im vec: chunk referenzen leben mindesten 'a lang -> so lang wie funktion
//3. 'a im return (->): ergebnis hängt von 'a lifetime also vom vec ab
//-> lifetime sagt nicht wie lange etwas lebt, sonder beschreibt beziehungen
pub fn chunks_spawn_enemy<'a>(chunks: Vec<&'a Chunk>, pool: &ChunkPool, chunk_num: f32, p_pos: IVec2) -> Vec<&'a Chunk> {
    let p_pos = p_pos.as_vec2();
    let min_x  = (p_pos.x - chunk_num).floor() as i32;
    let min_y = (p_pos.y - chunk_num).floor() as i32;
    let max_x = (p_pos.x + chunk_num).ceil() as i32;
    let max_y = (p_pos.y + chunk_num).ceil() as i32;

    chunks
        .into_iter()
        .filter(
            |c|
            c.cords.x >= min_x && c.cords.y >= min_y && c.cords.x <= max_x && c.cords.y <= max_y
        )
        .collect()
}

pub fn find_enemy_spawn_point(p_pos: Vec3, chunks: Vec<&Chunk>) -> Vec3 {
    //let chunk = chunks.
    p_pos
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