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

    for cords in &region.connection_tiles {
        if cords.x == x && cords.y == y { return Tile::Floor; }
    }

    return init_filler_tile;
}

fn set_region_connections(cords: Cords) -> Vec<Cords> {
    let mut connections: Vec<Cords> = Vec::new();
    connections.push(Cords {x: cords.x, y: cords.y});
    connections.push(Cords {x: cords.x+REGION_TILE_SIZE, y: cords.y+REGION_TILE_SIZE});
    connections
}

pub fn generate_region(
    world_seed: u64,
    x: i32,
    y: i32,
) -> Region {

    let cords = Cords {x, y};
    let rooms = generate_rooms(&cords, world_seed);
    //println!("generated init rooms:\n{:?}", rooms);
    let paths = generate_room_paths(&rooms, world_seed);
    let connection_tiles = set_region_connections(cords);

    println!("region boundings: ({:?}) -> ({:?})", cords, Cords {x: cords.x+REGION_TILE_SIZE, y: cords.y+REGION_TILE_SIZE});

    Region {
        cords,
        rooms,
        paths,
        connection_tiles,
    }
}
