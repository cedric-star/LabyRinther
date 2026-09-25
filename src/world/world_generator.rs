use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::world::seed_service::*;

pub fn load_map(
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    mut pool: ResMut<ChunkPool>,
    asset_server: Res<AssetServer>,
    seed: Res<WorldSeed>
) {

    for player in players {

        let player_x = player.translation.x;
        let player_y = player.translation.y;

        let region = generate_region(seed.seed, player_x as i32, player_y as i32);

        let chunk_cords = Cords::new(
            (player_x / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32,
            (player_y / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32,
        );

        if !pool.chunks.contains_key(&(chunk_cords.x, chunk_cords.y)) {
            println!("spawning chunk: {}:{}", player_x, player_y);

            let floor_handle: Handle<Image> = asset_server.load("ground_simple.png");
            let wall_handle: Handle<Image> = asset_server.load("wall_simple.png");
            let chunk_seed = calc_chunk_seed(seed.seed, player_x as i32, player_y as i32);
            let tiles = gen_tiles_by_seed(chunk_seed);

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
                for (local_y, row) in tiles.iter().enumerate() {
                    for (local_x, tile) in row.iter().enumerate() {
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
                        ));
                    }
                }
            });

            pool.chunks.insert((chunk_cords.x, chunk_cords.y), chunk_entity);

        }
    }
}

fn gen_tiles_by_seed(seed: u64) -> [[Tile; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut tile: [[Tile; CHUNK_SIZE]; CHUNK_SIZE] = [[Tile::Floor; CHUNK_SIZE]; CHUNK_SIZE];
    println!("seed: {seed}");
    for i in 0..CHUNK_SIZE  {
        for j in 0..CHUNK_SIZE {
            if is_even(calc_tile_seed(seed, i as i32, j as i32)) {
                tile[i][j] = Tile::Wall;
            } else {
                tile[i][j] = Tile::Floor;
            }
        }
    }

    tile
}

fn is_even(num: u64) -> bool {
    num as usize % 2 == 0
}

pub fn generate_region(
    world_seed: u64,
    region_x: i32, //mittelpunk der region
    region_y: i32,
) -> Region {
    let region_seed = calc_region_seed(
        world_seed,
        region_x,
        region_y,
    );

    let rooms = generate_rooms(region_seed);
//    let connections = connect_rooms(region_seed, &rooms);

    Region {
        x: region_x,
        y: region_y,
        rooms,
        //connections,
    }
}

fn room_type(type_seed: u64) -> RoomType {
    let rest = type_seed as usize % 10;

    match rest {
        0..=5 => RoomType::SMALL,
        6..=8 => RoomType::NORMAL,
        9 => RoomType::LARGE,
        _ => RoomType::SMALL,
    }
}

fn room_size(room_type: RoomType) -> (i32, i32) {
    match room_type {
        RoomType::SMALL => (6, 4),
        RoomType::NORMAL => (10, 8),
        RoomType::LARGE => (16, 10),
    }
}

fn generate_rooms(seed: u64) -> Vec<Room> {
    let room_count = (seed as usize % 4) + 2;

    let mut rooms = Vec::new();

    for id in 0..room_count {
        let room_seed = calc_room_seed(seed, id as i32);
        let room_type = room_type(room_seed);
        let (width, height) = room_size(room_type);

        let x = calc_room_seed(seed, width) as i32 % REGION_WORLD_SIZE;
        let y = calc_room_seed(seed, height) as i32 % REGION_WORLD_SIZE;

        let entry = direction_from_seed(room_seed);

        rooms.push(Room {
            id: RoomId(id as i32),
            x,
            y,
            width,
            height,
            entry,
            room_type,
        });
    }
    rooms

}

fn direction_from_seed(seed: u64) -> Direction {
    match seed % 4 {
        0 => Direction::NORTH,
        1 => Direction::EAST,
        2 => Direction::SOUTH,
        3 => Direction::WEST,
        _ => unreachable!(),
    }
}

fn rooms_overlap(a: &Room, b: &Room) -> bool {
    let padding = 2;

    a.x - padding < b.x + b.width
        && a.x + a.width + padding > b.x
        && a.y - padding < b.y + b.height
        && a.y + a.height + padding > b.y
}
