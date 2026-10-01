use bevy::prelude::*;
use crate::world::room::*;
use crate::constants::*;
use crate::components::*;
use crate::util::seed_service::*;
use crate::util::finder::*;

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

fn set_region_connections(cords: Cords, seed: u64) -> Vec<Cords> {
    let mut connections: Vec<Cords> = Vec::new();

    let directions: Vec<Direction> = vec![Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];
    for (i, direction) in directions.iter().enumerate() {
        let direction_seed = calc_any_hash(seed, i as i32);

        let num = (direction_seed % (MAX_REGION_SIDE_CONNECTIONS - MIN_REGION_SIDE_CONNECTIONS) + MIN_REGION_SIDE_CONNECTIONS);

        for i in 0..num {
            let iter_seed = calc_any_hash(direction_seed, i as i32);
            let val = ((iter_seed % (REGION_TILE_SIZE - 8) as u64) + 4) as i32;

            match direction {
                Direction::NORTH => {connections.push(Cords { x: cords.x + val, y: cords.y + REGION_TILE_SIZE}); }

                Direction::EAST => {connections.push(Cords { x: cords.x + REGION_TILE_SIZE, y: cords.y + val}); }

                Direction::SOUTH => {connections.push(Cords { x: cords.x + val, y: cords.y}); }

                Direction::WEST => {connections.push(Cords { x: cords.x, y: cords.y + val}); }
            }
        }
    }

    connections
}

fn connect_connections_2_paths(current_paths: &Vec<Cords>, connections: &Vec<Cords>, region_bounds: Cords, seed: u64, rooms: &Vec<Room>) -> Vec<Cords> {
    let mut paths: Vec<Cords> = Vec::new();

    for connection in connections {
        let target = find_nearest(connection, current_paths);

        println!("starting searching {:?} -> {:?}", connection, target);

        let my_seed = calc_any_hash_2(seed, connection.x, connection.y);

        let mut connection_path = my_search((*connection, target), rooms, my_seed, region_bounds);
        paths.append(&mut connection_path);
    }

    paths
}

pub fn find_nearest(c: &Cords, cords: &Vec<Cords>) -> Cords {
    match cords.iter().min_by_key(|target| manhattan(c, target)) {
        Some(goal) => {return *goal;}
        None => {
            println!("found no connection between connection and path");
            return *c;
        }
    }

}

pub fn generate_region(
    world_seed: u64,
    x: i32,
    y: i32,
) -> Region {

    let cords = Cords {x, y};
    let rooms = generate_rooms(&cords, world_seed);
    //println!("generated init rooms:\n{:?}", rooms);
    let mut paths = generate_room_paths(&rooms, world_seed, cords);

    let connection_tiles = set_region_connections(cords, world_seed);

    paths.append(&mut connect_connections_2_paths(&paths, &connection_tiles, cords, world_seed, &rooms));

    println!("region boundings: ({:?}) -> ({:?})", cords, Cords {x: cords.x+REGION_TILE_SIZE, y: cords.y+REGION_TILE_SIZE});

    Region {
        cords,
        rooms,
        paths,
        connection_tiles,
    }
}
