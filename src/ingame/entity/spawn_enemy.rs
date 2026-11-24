use bevy::prelude::*;
use rand::prelude::*;
use avian2d::prelude::*;
use crate::ingame::player::structs::*;
use crate::settings::settings_screen::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::ingame::entity::structs::*;
use crate::ingame::animation::*;
use crate::ingame::structs::*;

pub fn set_enemies_num_res(
    mut enemies_res: ResMut<Enemies>,
    enemies: Query<&Enemy>,
) {
    let num = enemies.iter().count() as u32;
    if num != enemies_res.num {
        enemies_res.num = num;
    }
}

//system dass enemies spawnt
pub fn spawn_enemy_near_player(
    s: Res<GameSettings>,
    player: Single<&Transform, With<Player>>,
    region_pool: Res<RegionPool>,
    chunks: Query<&Chunk>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    spatial_query: SpatialQuery,
) {
    //player
    let p_pos = player.translation;
    let p_chunk_pos = IVec2::new(p_pos.x as i32, p_pos.y as i32).world_to_chunk();

    //region in dem player is
    let r_cords = IVec2::new(p_pos.x as i32, p_pos.y as i32).world_to_region();
    let Some(&region_entity) = region_pool.regions.get(&r_cords) else { return; };

    //all chunks as vec
    let chunk_vec: Vec<&Chunk> = chunks.iter().collect();

    //filtered: only chunks near player (number based on settings)
    let chunks_2_spawn_enemies: Vec<&Chunk> = chunks_spawn_enemy(chunk_vec, s.chunks_enemy_spawnable, p_chunk_pos);

    for _ in 1..=5 {
        //a world koordinate, where a place is free
        let Some(enemy_spawn_point) = find_enemy_spawn_point(p_pos, &chunks_2_spawn_enemies) else {continue;};

        //schauen ob da schon ein gegner ist
        if spatial_query.shape_intersections(
            &Collider::circle(PLAYER_WIDTH / 2.0), 
            enemy_spawn_point.truncate(),
            0.0,
            &SpatialQueryFilter::from_mask([
                Layer::Enemy,
                Layer::Player,
                Layer::Wall,
            ])
        ).is_empty() {// wenn frei dann spawnen und loop unterbrechen
            commands.spawn(get_enemy_bundle(
                enemy_spawn_point,
                &asset_server, 
                &mut texture_atlas_layouts
            ));
            break;
        }//wenn nicht insgesammt 5 mal neu probieren einen punkt zu finden
    }
}

//lifetime checker weis nicht, ob rückgabe chunks aus vec<chunks> oder chunkpool stammt -> fehler
//lsg: 1. lifetimeannotation: 'a in funktion erstellt lifetime namens a
//2. 'a im vec: chunk referenzen leben mindesten 'a lang -> so lang wie funktion
//3. 'a im return (->): ergebnis hängt von 'a lifetime also vom vec ab
//-> lifetime sagt nicht wie lange etwas lebt, sonder beschreibt beziehungen
fn chunks_spawn_enemy<'a>(chunks: Vec<&'a Chunk>, chunk_num: f32, p_pos: IVec2) -> Vec<&'a Chunk> {
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

//finde einen freien chunk in den verfügbaren chunks
fn find_enemy_spawn_point(p_pos: Vec3, chunks: &Vec<&Chunk>) -> Option<Vec3> {
    //let chunk = chunks.
    let mut rng = rand::rng();
    let chunk = chunks.choose(&mut rand::rng())?;

    let Some(tile_cords) = find_free_tile_in_chunk(*chunk) else {return None; };

    let tile_pos_in_chunk_middle_point = tile_cords.tile_to_world() + IVec2::splat((PLAYER_WIDTH / 2.0) as i32);
    let enemy_cords = (chunk.cords.chunk_to_world() + tile_pos_in_chunk_middle_point).extend(ZLayer::Enemy.z().floor() as i32);

    return Some(enemy_cords.as_vec3());
}

//finde ein tile in einem chunk das floor ist, und drumherum ein kreis aus floors
fn find_free_tile_in_chunk(chunk: &Chunk) -> Option<IVec2> {

    let mut possible_tiles: Vec<IVec2> = Vec::new();

    //chunk borders nicht spawnable machen, also last und 1 statt 0
    let last = CHUNK_SIZE as usize - 1;
    for col in 1..last {
        'tile: for row in 1..last {
            if chunk.tiles[col][row] != Tile::Floor { continue; }

            //3x3 tile umgebung prüfen
            for dx in -1_i32..=1 {
                for dy in -1_i32..=1 {
                    let c = (col as i32 + dx) as usize;
                    let r = (row as i32 + dy) as usize;

                    if chunk.tiles[c][r] != Tile::Floor {
                        continue 'tile;
                    }
                }
            }
            possible_tiles.push(IVec2::new(col as i32, row as i32));
        }
    }
    possible_tiles.choose(&mut rand::rng()).copied()

}


/// eigentliches bundle für enemy:
pub fn get_enemy_bundle(
    pos: Vec3,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,

) -> impl Bundle {

    let clips = Player::get_clips(asset_server, texture_atlas_layouts);
    let anim = clips.clips.get(&Anim::Humanoid(HumanoidState::StandingFront)).unwrap();
    let timer = AnimationTimer::new();
    
    (
        RigidBody::Dynamic,
        Collider::circle(PLAYER_WIDTH / 2.0),
        CollisionLayers::new(Layer::Enemy, [Layer::Wall, Layer::Player, Layer::PlayerWeapon, Layer::Enemy]),
        LinearDamping(20.0),
        LockedAxes::ROTATION_LOCKED,//damit sich gegner nicht hässlich dreht
        GameEntity,
        Enemy::new(),
        Transform::from_translation(pos),
        Visibility::Visible,
        MyEntity,
        Anim::Humanoid(HumanoidState::StandingFront),
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: anim.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: anim.layout.clone(),
                index: anim.first.clone(),
            }),
            ..default()
        },
        clips,
        timer,
        children![
            (
                Sprite { custom_size: Some(Vec2::new(35.0, 2.0)), color: Color::srgb(1.0, 0.0, 0.0), ..default() },
                Transform::from_xyz(0.0, (PLAYER_HEIGHT / 2.0) + 2.0, ZLayer::EnemyHealthBarForeground.z()),
                EnemyHealthBar,
            ),
            (
                Sprite { custom_size: Some(Vec2::new(35.0, 2.0)), color: Color::srgb(0.5, 0.1, 0.1), ..default() },
                Transform::from_xyz(0.0, (PLAYER_HEIGHT / 2.0) + 2.0, ZLayer::EnemyHealthBarBackground.z()),
                EnemyHealthBarBackground,
            ),
        ],
    )
}