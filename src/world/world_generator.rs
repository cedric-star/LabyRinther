use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::util::seed_service::*;
use crate::world::region::*;
use crate::settings::settings_screen::*;
pub fn load_map(
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    mut pool: ResMut<ChunkPool>,
    asset_server: Res<AssetServer>,
    seed: Res<WorldSeed>,
    mut regions: ResMut<RegionPool>,
    region_query: Query<&Region>,
    s: Res<GameSettings>,
) {
    for player in &players {
        let player_chunk_x = (player.translation.x / CHUNK_WORLD_SIZE).floor() as i32;
        let player_chunk_y = (player.translation.y / CHUNK_WORLD_SIZE).floor() as i32;

        let player_chunks: Vec<Cords> = players
            .iter()
            .map(|t| Cords::new(
                (t.translation.x / CHUNK_WORLD_SIZE).floor() as i32,
                (t.translation.y / CHUNK_WORLD_SIZE).floor() as i32,
            ))
            .collect();

        let out_of_scope: Vec<Cords> = pool
            .chunks
            .keys()
            .copied()
            .filter(|c| {
                !player_chunks.iter().any(|p| {
                    (c.x - p.x).abs() <= s.chunks_loading && (c.y - p.y).abs() <= s.chunks_loading
                })
            })
            .collect();

        for c in out_of_scope {
            if let Some(entity) = pool.chunks.remove(&c) {
                commands.entity(entity).despawn();
            }
        }
        for c_view_x in -s.chunks_loading..=s.chunks_loading {
            for c_view_y in -s.chunks_loading..=s.chunks_loading {
                let chunk_cords = Cords::new(player_chunk_x + c_view_x, player_chunk_y + c_view_y);

                if pool.chunks.contains_key(&chunk_cords) {
                    continue;
                }

                // Region, zu der dieser Chunk gehört
                let region_key = Cords::new(
                    chunk_cords.x.div_euclid(REGION_SIZE),
                    chunk_cords.y.div_euclid(REGION_SIZE),
                );

                let region_entity = regions.regions.get(&region_key).copied();
                let Some(region_entity) = region_entity else {
                    let e = commands
                        .spawn((GameEntity, generate_region(seed.seed, region_key)))
                        .id();
                    regions.regions.insert(region_key, e);
                    continue; // Region ist erst im nächsten Frame abfragbar
                };
                let Ok(region) = region_query.get(region_entity) else {
                    continue;
                };

                let tiles = gen_tiles(seed.seed, region, chunk_cords);
                let chunk_world_pos = chunk_cords.chunk_to_world();

                let chunk_entity = commands
                    .spawn((
                        GameEntity,
                        Chunk { tiles, cords: chunk_cords },
                        Transform::from_xyz(chunk_world_pos.x, chunk_world_pos.y, 0.0),
                        Visibility::default(),
                    ))
                    .id();

                commands.entity(chunk_entity).with_children(|parent| {
                    for (local_x, row) in tiles.iter().enumerate() {
                        for (local_y, tile) in row.iter().enumerate() {
                            parent.spawn((
                                GameEntity,
                                Sprite {
                                    image: asset_server.load(tile.asset()),
                                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                    ..default()
                                },
                                Transform::from_xyz(
                                    local_x as f32 * TILE_SIZE,
                                    local_y as f32 * TILE_SIZE,
                                    0.0,
                                ),
                                Anchor::BOTTOM_LEFT,
                            ));
                        }
                    }
                });

                pool.chunks.insert(chunk_cords, chunk_entity);
            }
        }
    }
}
fn gen_tiles(seed: u64, region: &Region, chunk_cords: Cords) -> [[Tile; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE] = [[Tile::Wall; CHUNK_SIZE]; CHUNK_SIZE];

    for local_x in 0..CHUNK_SIZE {
        for local_y in 0..CHUNK_SIZE {

            let world_x = chunk_cords.x * CHUNK_SIZE as i32 + local_x as i32;
            let world_y = chunk_cords.y * CHUNK_SIZE as i32 + local_y as i32;

            tiles[local_x][local_y] = set_region_tiles(world_x, world_y, &region);
        }
    }

    tiles
}

fn is_even(num: u64) -> bool {
    num as usize % 2 == 0
}
