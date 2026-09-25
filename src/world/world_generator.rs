use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::world::seed_service::*;

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
                    println!("player at absolute: {player_x}:{player_y}");
                    println!("player at tile: {}:{}", (player_x / TILE_SIZE).floor(), (player_y / TILE_SIZE).floor());
                    println!("player in chunk: {}:{}", chunk_cords.x, chunk_cords.y);
                    println!(" ");

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

            tiles[local_x][local_y] = if is_room_wall(world_x as i32, world_y as i32, region) {
                Tile::Wall
            } else {
                Tile::Floor
            };
        }
    }

    tiles
}

fn is_room_wall(local_x: i32, local_y: i32, region: &Region) -> bool {
    for room in &region.rooms {
        let left   = room.x;
        let right  = room.x + room.width - 1;
        let top    = room.y + room.height - 1;
        let bottom = room.y;

        let on_vertical   = (local_x == left || local_x == right)
                         && local_y >= bottom && local_y <= top;
        let on_horizontal = (local_y == top || local_y == bottom)
                         && local_x >= left && local_x <= right;

        if !on_vertical && !on_horizontal {
            continue; // Tile gehört nicht zu diesem Room
        }

        // Ecken sind immer Wand
        let is_corner = (local_x == left || local_x == right)
                     && (local_y == top  || local_y == bottom);
        if is_corner {
            return true;
        }

        // Bestimme, auf welcher Seite die Tile liegt
        let side = if local_y == top    { Direction::NORTH }
                   else if local_y == bottom { Direction::SOUTH }
                   else if local_x == left   { Direction::WEST  }
                   else                      { Direction::EAST  };

        // Wenn diese Seite der Entry ist → keine Wand
        let temp_cords = Cords::new(local_x, local_y);
        if temp_cords == room.entry_cords { return false; }
        return true;

    }
    false
}

fn is_even(num: u64) -> bool {
    num as usize % 2 == 0
}

pub fn generate_region(
    world_seed: u64,
    mid_tile_cords: Cords
) -> Region {
    let region_seed = calc_region_seed(
        world_seed,
        mid_tile_cords.x,
        mid_tile_cords.y,
    );

    let rooms = generate_rooms(region_seed);
//    let connections = connect_rooms(region_seed, &rooms);

    Region {
        cords: mid_tile_cords,
        rooms,
        //connections,
    }
}

pub fn generate_rooms(seed: u64) -> Vec<Room> {
    let room_count = (seed as usize % 2) + 2;

    let mut rooms = Vec::new();

    for id in 0..room_count {
        let id = id as i32;
        let room_seed = calc_room_seed(seed, id, id);
        let room_type = room_type(room_seed);
        let (width, height) = room_type.dimensions();

        let x = calc_room_seed(seed, id, width) as i32 % REGION_TILE_SIZE;
        let y = calc_room_seed(seed, id, height) as i32 % REGION_TILE_SIZE;

        let entry = direction_from_seed(room_seed);
        let entry_cords = entry_cords_from_direction_and_type(room_seed, x, y, room_type, entry);

        rooms.push(Room {
            id: RoomId(id as i32),
            x,
            y,
            width,
            height,
            entry,
            entry_cords,
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

fn entry_cords_from_direction_and_type(
    seed: u64,
    x: i32,
    y: i32,
    room_type: RoomType,
    entry: Direction,
) -> Cords {
    let (width, height) = room_type.dimensions();
    let nums: (i32, i32) = match entry {
        Direction::SOUTH => {
            let offset = (seed % (width as u64 - 2)) as i32 + 1;
            (x + offset, y)
        }
        Direction::NORTH => {
            let offset = (seed % (width as u64 - 2)) as i32 + 1;
            (x + offset, y + height - 1)
        }
        Direction::WEST => {
            let offset = (seed % (height as u64 - 2)) as i32 + 1;
            (x, y + offset)
        }
        Direction::EAST => {
            let offset = (seed % (height as u64 - 2)) as i32 + 1;
            (x + width - 1 , y + offset)
        }
    };
    Cords::new(nums.0, nums.1)
}
fn rooms_overlap(a: &Room, b: &Room) -> bool {
    let padding = 2;

    a.x - padding < b.x + b.width
        && a.x + a.width + padding > b.x
        && a.y - padding < b.y + b.height
        && a.y + a.height + padding > b.y
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
