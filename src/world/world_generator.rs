use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::world::seed_service::*;
use crate::world::room::*;
use crate::world::region::*;

pub fn load_map(
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    mut pool: ResMut<ChunkPool>,
    asset_server: Res<AssetServer>,
    seed: Res<WorldSeed>,
    regions: Res<RegionPool>,
    region_query: Query<&Region>
) {

    for player in players {

        let player_x = player.translation.x;
        let player_y = player.translation.y;

        for c_view_x in -CHUNKS_LOADING..=CHUNKS_LOADING {
            for c_view_y in -CHUNKS_LOADING..=CHUNKS_LOADING {

                let chunk_cords = Cords::new(
                    (player_x / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32 + c_view_x as i32,
                    (player_y / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32 + c_view_y as i32,
                );

                let region_entity = regions.regions.get(&(0, 0)).unwrap();
                let region = region_query.get(*region_entity).unwrap();

                if !pool.chunks.contains_key(&(chunk_cords.x, chunk_cords.y)) {
                    //println!("player at absolute: {player_x}:{player_y}");
                    //println!("player at tile: {}:{}", (player_x / TILE_SIZE).floor(), (player_y / TILE_SIZE).floor());
                    //println!("player in chunk: {}:{}", chunk_cords.x, chunk_cords.y);
                    //println!(" ");

                    let floor_handle: Handle<Image> = asset_server.load("ground_simple.png");
                    let wall_handle: Handle<Image> = asset_server.load("wall_simple.png");
                    let chunk_seed = calc_chunk_seed(seed.seed, player_x as i32, player_y as i32);
                    let tiles = gen_tiles(chunk_seed, &region, chunk_cords);

                    let chunk_world_pos = chunk_cords.chunk_to_world();

                    let chunk = Chunk {
                        tiles: tiles,
                        cords: chunk_cords,

                    };

                    let chunk_entity = commands.spawn((
                        chunk,
                        Transform::from_xyz(
                            chunk_world_pos.x,
                            chunk_world_pos.y,
                            0.0,
                        ),
                        Visibility::default(),
                    )).id();

                    commands.entity(chunk_entity).with_children(|parent| {
                        for (local_x, row) in tiles.iter().enumerate() {
                            for (local_y, tile) in row.iter().enumerate() {
                                // Berechne World-Position
                                let world_x = (local_x as f32) * TILE_SIZE;
                                let world_y = (local_y as f32) * TILE_SIZE;

                                let texture = match tile {
                                    Tile::Floor => asset_server.load("ground_simple.png"),
                                    Tile::Wall => asset_server.load("wall_simple.png"),
                                };

                                // Spawn Tile-Entity
                                parent.spawn((
                                    Sprite {
                                        image: texture,
                                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                        ..default()
                                    },
                                    Transform::from_xyz(world_x, world_y, 0.0),
                                    Anchor::BOTTOM_LEFT,
                                ));
                            }
                        }
                    });

                    pool.chunks.insert((chunk_cords.x, chunk_cords.y), chunk_entity);
                }
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
