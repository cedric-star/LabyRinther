use bevy::prelude::*;
use bevy::sprite::*;
use avian2d::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::world::region::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::structs::*;

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


                //collider aus tile walls für chunk bauen
                let collider = build_chunk_collider(&tiles);

                let mut chunk_cmd = commands
                    .spawn((
                        GameEntity,
                        Chunk { tiles, cords: chunk_cords },
                        Transform::from_xyz(chunk_world_pos.x, chunk_world_pos.y, 0.0),
                        Visibility::default(),
                        RigidBody::Static,
                    ));
                if let Some(c) = collider {
                    chunk_cmd.insert(c);
                }
                let chunk_entity = chunk_cmd.id();

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


fn build_chunk_collider(tiles: &[[Tile; CHUNK_SIZE]; CHUNK_SIZE]) -> Option<Collider> {
    let mut parts: Vec<(Vec2, f32, Collider)> = Vec::new();

    for x in 0..CHUNK_SIZE {
        let mut y = 0;
        while y < CHUNK_SIZE {
            if tiles[x][y] == Tile::Wall {
                let start = y;
                while y < CHUNK_SIZE && tiles[x][y] == Tile::Wall {
                    y += 1;
                }
                let len = (y - start) as f32;

                // lokal zum Chunk-Entity, Mittelpunkt des Rechtecks
                let center = Vec2::new(
                    (x as f32 + 0.5) * TILE_SIZE,
                    (start as f32 + len / 2.0) * TILE_SIZE,
                );
                parts.push((center, 0.0, Collider::rectangle(TILE_SIZE, len * TILE_SIZE)));
            } else {
                y += 1;
            }
        }
    }

    if parts.is_empty() { None } else { Some(Collider::compound(parts)) }
}