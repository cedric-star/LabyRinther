use bevy::prelude::*;
use crate::world::room::*;
use crate::constants::*;
use crate::components::*;

pub fn set_region_tiles(x: i32, y: i32, region: &Region) -> Tile {
    let init_filler_tile = Tile::Wall;

    for room in &region.rooms {
        let room_tile = set_room_tile(x, y, room);

        if room_tile != init_filler_tile { return room_tile; }
    }

    for cord in &region.paths {
        if cord.x == x && cord.y == y { return Tile::Floor; }
    }

    return init_filler_tile;
}

pub fn generate_region(
    world_seed: u64,
    x: i32,
    y: i32,
) -> Region {

    let cords = Cords {x, y};
    let rooms = generate_rooms(world_seed);
    println!("generated init rooms:\n{:?}", rooms);
    let paths = generate_room_paths(&rooms, world_seed);

    Region {
        cords,
        rooms,
        paths,
    }
}
