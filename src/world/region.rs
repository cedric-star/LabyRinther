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

    for (_, cords) in &region.connection_tiles {
        if cords.x == x && cords.y == y { return Tile::Floor; }
    }

    return init_filler_tile;
}

fn set_region_connections(cords: Cords, seed: u64) -> Vec<(Direction, Cords)> {
    let mut connections: Vec<(Direction, Cords)> = Vec::new();

    let directions: Vec<Direction> = vec![Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];
    for (i, direction) in directions.iter().enumerate() {
        let direction_seed = calc_any_hash(seed, i as i32);

        let num = (direction_seed % (MAX_REGION_SIDE_CONNECTIONS - MIN_REGION_SIDE_CONNECTIONS) + MIN_REGION_SIDE_CONNECTIONS);

        for i in 0..num {
            let iter_seed = calc_any_hash(direction_seed, i as i32);
            let val = ((iter_seed % (REGION_TILE_SIZE - 8) as u64) + 4) as i32;

            match direction {
                Direction::NORTH => {connections.push((*direction, Cords { x: cords.x + val, y: cords.y + REGION_TILE_SIZE})); }

                Direction::EAST => {connections.push((*direction, Cords { x: cords.x + REGION_TILE_SIZE, y: cords.y + val})); }

                Direction::SOUTH => {connections.push((*direction, Cords { x: cords.x + val, y: cords.y})); }

                Direction::WEST => {connections.push((*direction, Cords { x: cords.x, y: cords.y + val})); }
            }
        }
    }

    connections
}

fn connect_connections_2_paths(current_paths: &Vec<Cords>, connections: &Vec<(Direction, Cords)>, region_bounds: Cords, seed: u64, rooms: &Vec<Room>) -> Vec<Cords> {
    let mut paths: Vec<Cords> = Vec::new();

    for (direction, connection) in connections {
        let mut connection = connection.clone();
        match direction {
            Direction::NORTH => { connection.y -= PATH_PADDING + 1; }
            Direction::EAST => { connection.x -= PATH_PADDING + 1; }
            Direction::SOUTH => { connection.y += PATH_PADDING + 1; }
            Direction::WEST => { connection.x += PATH_PADDING + 1; }
        }
        let target = find_nearest(&connection, current_paths);

        println!("starting searching {:?} -> {:?}", connection, target);

        let my_seed = calc_any_hash_2(seed, connection.x, connection.y);

        let mut connection_path = my_search((connection, target), rooms, my_seed, region_bounds, true);
        paths.append(&mut connection_path);

        println!("\n");
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

//returns x,y and x+w,y+h in a tupel
fn find_empty_spaces(paths: &Vec<Cords>, region_cords: Cords, n: usize) -> Vec<(Cords, Cords)> {
    let size = REGION_TILE_SIZE as usize;
    let mut grid = vec![vec![false; size]; size]; // grid[y][x], true = belegt

    for p in paths {
        let gx = p.x - region_cords.x;
        let gy = p.y - region_cords.y;
        if gx >= 0 && gy >= 0 && (gx as usize) < size && (gy as usize) < size {
            grid[gy as usize][gx as usize] = true;
        }
    }

    let mut spaces: Vec<(Cords, Cords)> = Vec::new();

    for _ in 0..n {
        match largest_empty_rect(&grid, size) {
            Some((x0, y0, x1, y1)) => {
                // Fläche als belegt markieren, damit das nächste Rechteck disjunkt ist
                for y in y0..y1 {
                    for x in x0..x1 {
                        grid[y][x] = true;
                    }
                }
                spaces.push((
                    Cords { x: region_cords.x + x0 as i32, y: region_cords.y + y0 as i32 },
                    Cords { x: region_cords.x + x1 as i32, y: region_cords.y + y1 as i32 },
                ));
            }
            None => break, // nichts Freies mehr
        }
    }

    spaces
}

//largest free rectangle in grid, returns (x0, y0, x1, y1) with x1/y1 exclusive
fn largest_empty_rect(grid: &Vec<Vec<bool>>, size: usize) -> Option<(usize, usize, usize, usize)> {
    let mut heights = vec![0usize; size];
    let mut best_area = 0;
    let mut best = None;

    for y in 0..size {
        // Histogramm: wie viele freie Tiles liegen direkt übereinander (bis Zeile y)
        for x in 0..size {
            heights[x] = if grid[y][x] { 0 } else { heights[x] + 1 };
        }

        // größtes Rechteck im Histogramm mit Stack (Index, Höhe)
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for x in 0..=size {
            let h = if x < size { heights[x] } else { 0 }; // 0 am Ende leert den Stack
            let mut start = x;

            while let Some(&(sx, sh)) = stack.last() {
                if sh <= h { break; }
                stack.pop();
                let area = sh * (x - sx);
                if area > best_area {
                    best_area = area;
                    // Rechteck endet in Zeile y (inklusive), ist sh hoch und reicht von sx bis x
                    best = Some((sx, y + 1 - sh, x, y + 1));
                }
                start = sx;
            }
            stack.push((start, h));
        }
    }

    best
}

//sucht nach größeren flächen und mahlt senk- und waagerechte verbindungen (lange strecken)
fn set_connection_lines(cords: &Vec<Cords>, region_cords: Cords, seed: u64) -> Vec<Cords> {
    let mut arr: Vec<Cords> = Vec::new();
    for mut square in find_empty_spaces(cords, region_cords, EMPTY_ROOMS) {
        let path_num = (calc_any_hash_2(seed, square.0.x, square.0.y) % (MAX_EMPTY_PATHS - MIN_EMPTY_PATHS + 1)) + MIN_EMPTY_PATHS;

        println!("generated square: {:?}, path_num: {path_num}", square);
        square.0.x += PATH_PADDING + 1;
        square.0.y += PATH_PADDING + 1;
        square.1.x -= PATH_PADDING + 1;
        square.1.y -= PATH_PADDING + 1;

        let width = square.1.x - square.0.x;
        let height = square.1.y - square.0.y;

        for i in 0..path_num as i32 {
            let x = (calc_any_hash_2(seed, width, i) % width as u64) as i32 + square.0.x;
            let y = (calc_any_hash_2(seed, height, i) % height as u64) as i32 + square.0.y;

            arr.push(Cords { x: x as i32, y: y as i32});

        }
    }
    arr
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

    let mut more_connections = set_connection_lines(&paths, cords, world_seed);

    println!("region boundings: ({:?}) -> ({:?})", cords, Cords {x: cords.x+REGION_TILE_SIZE, y: cords.y+REGION_TILE_SIZE});

    paths.append(&mut more_connections);

    Region {
        cords,
        rooms,
        paths,
        connection_tiles,
    }
}
