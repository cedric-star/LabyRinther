use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::util::seed_service::*;
use crate::world::region::*;
use std::collections::{VecDeque, HashMap, HashSet};

pub fn generate_rooms(region_cords: &Cords, seed: u64) -> Vec<Room> {
    let room_count = (seed as usize % 2) + 2;

    let mut rooms = Vec::new();

    for id in 0..room_count {
        let id = id as i32;
        let room_seed = calc_room_seed(seed, id, id);
        let room_type = room_type(room_seed);
        let (width, height) = room_type.dimensions();

        let x = region_cords.x + (calc_room_seed(seed, id, width) as i32 % REGION_TILE_SIZE).abs();
        let y = region_cords.y + (calc_room_seed(seed, id, height) as i32 % REGION_TILE_SIZE).abs();

        let entry = direction_from_seed(room_seed);
        let entry_cords = entry_cords_from_direction_and_type(room_seed, x, y, room_type, entry);

        rooms.push(Room {
            id: RoomId(id as i32),
            cords: Cords {x, y},
            width,
            height,
            entry,
            entry_cords,
            room_type,
        });
    }
    rooms

}

//nimm alle räume und erstelle eine liste an koordinaten
//-> gemalte koordinaten sind pfad
pub fn generate_room_paths(rooms: &Vec<Room>, seed: u64) -> Vec<Cords> {
    let mut paths: Vec<Cords> = Vec::new();
    for (index, room) in rooms.iter().enumerate() {
        let next_room = match rooms.get(index + 1) {
            Some(room) => room,
            None => { break; },
        };
        let mut starts = get_moved_entry_cords(room);
        let start_cords = starts.last().unwrap();
        let mut targets = get_moved_entry_cords(next_room);
        let target_cords = targets.last().unwrap();

        println!("starts: {:?}", starts);
        paths.append(&mut starts.clone());
        paths.append(&mut targets.clone());

        paths.append(&mut my_search((start_cords.clone(), target_cords.clone()), &rooms, seed));
    }

    paths
}

fn get_moved_entry_cords(room: &Room) -> Vec<Cords> {
    let mut cords = Vec::new();
    match room.entry {
            Direction::NORTH => {
                for i in 1..=(PATH_PADDING + 1) {
                     cords.push(Cords {x: room.entry_cords.x, y: room.entry_cords.y + i});
                }
            },
            Direction::EAST => {
                for i in 1..=(PATH_PADDING + 1) {
                     cords.push(Cords {x: room.entry_cords.x + i, y: room.entry_cords.y});
                }
            }
            Direction::SOUTH => {
                for i in 1..=(PATH_PADDING + 1) {
                     cords.push(Cords {x: room.entry_cords.x, y: room.entry_cords.y - i});
                }
            },
            Direction::WEST => {
                for i in 1..=(PATH_PADDING + 1) {
                     cords.push(Cords {x: room.entry_cords.x - i, y: room.entry_cords.y});
                }
            },
    };
    cords
}

fn manhattan(a: &Cords, b: &Cords) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn is_blocked(c: Cords, rooms: &[Room], start: Cords, target: Cords) -> bool {

    if c == start || c == target {
        return false;
    }

//    if blocked_cords.iter().any(|bc| c == *bc) { return true; }

    rooms.iter().any(|r| {
        c.x >= (r.cords.x - PATH_PADDING) && c.x < (r.cords.x + r.width + PATH_PADDING) &&
        c.y >= (r.cords.y - PATH_PADDING) && c.y < (r.cords.y + r.height + PATH_PADDING)
    })

}

fn get_neighnbour_cords(cords: Cords, padding: i32) -> Vec<Cords> {
        let mut neighbours: Vec<Cords> = Vec::new();
    for dx in -padding..=padding {
        for dy in -padding..=padding {
            if dx == 0 && dy == 0 { continue; }

            neighbours.push(Cords {x: cords.x + dx, y: cords.y + dy})
        }
    }
    neighbours
}

fn get_touching_neighnbour_cords(cords: Cords) -> Vec<Cords> {
    let mut neighbours: Vec<Cords> = Vec::new();
    let i = 1;

        neighbours.push(Cords { x: cords.x, y: cords.y + i });
        neighbours.push(Cords {x: cords.x + i, y: cords.y });
        neighbours.push(Cords { x: cords.x, y: cords.y - i});
        neighbours.push(Cords { x: cords.x - i, y: cords.y});

    neighbours
}

fn calc_noise(seed: u64, cords: &Cords) -> i32 {
    (calc_any_hash_2(seed, cords.x, cords.y) % NOISE_RANGE as u64) as i32
}

fn straight_run(path: &[Cords]) -> (i32, i32, i32) {
    let n = path.len();
    if n < 2 {
        return (0, 0, 0);
    }
    let dx = path[n - 1].x - path[n - 2].x;
    let dy = path[n - 1].y - path[n - 2].y;
    let mut run = 1;
    let mut i = n - 1;
    while i >= 2
        && path[i - 1].x - path[i - 2].x == dx
        && path[i - 1].y - path[i - 2].y == dy
    {
        run += 1;
        i -= 1;
    }
    (dx, dy, run)
}

pub fn my_search(cords: (Cords, Cords), all_rooms: &Vec<Room>, seed: u64) -> Vec<Cords> {
    let start = cords.0;
    let target = cords.1;

    let mut visited: Vec<Cords> = vec![start];          // aktueller Weg
    let mut seen: HashSet<Cords> = HashSet::new();      // alles je Besuchte, wird NIE gekürzt
    let mut way_padding: Vec<Cords> = Vec::new();
    seen.insert(start);

    // Fallback: bester Stand, falls kein Weg gefunden wird
    let mut best = visited.clone();
    let mut best_dist = manhattan(&start, &target);

    println!("find way: {:?} -> {:?}", start, target);

    let mut counter = 0;

    loop {
        let last = *visited.last().unwrap();
        if last == target {
            break;
        }

        counter += 1;
        if counter >= 100000 {
            visited = best;
            break;
        }

        let mut candidates: Vec<Cords> = get_touching_neighnbour_cords(last)
            .into_iter()
            .filter(|n| !seen.contains(n))
            .filter(|n| !is_blocked(*n, all_rooms, start, target))
            .collect();

            // Abbiegen verbieten, solange der gerade Abschnitt noch zu kurz ist
        let (dx, dy, run) = straight_run(&visited);
        if run > 0 && run < MIN_STRAIGHT {
            let ahead = Cords { x: last.x + dx, y: last.y + dy };
            // nur erzwingen, wenn geradeaus möglich ist und nicht vom Ziel wegführt
            if candidates.contains(&ahead) && manhattan(&ahead, &target) <= manhattan(&last, &target) {
                candidates = vec![ahead];
            }
        }

        let my_cords = candidates
            .into_iter()
            .min_by_key(|r| manhattan(r, &target) + calc_noise(seed, r));

        match my_cords {
            Some(c) => {
                seen.insert(c);
            visited.push(c);

                let d = manhattan(&c, &target);
                if d < best_dist {
                    best_dist = d;
                    best = visited.clone();
                }
            }
            None => {
                // Sackgasse: einen Schritt zurück, seen bleibt unverändert
                visited.pop();
                if visited.is_empty() {
                    visited = best;   // alles erschöpft -> Teilweg zurückgeben
                    break;
                }
            }
        }
    }

    // Padding erst am Ende, nur für den tatsächlichen Weg
    for c in &visited {
        way_padding.append(&mut get_neighnbour_cords(*c, PATH_PADDING));
    }
    visited.append(&mut way_padding);
    visited
}

//checke, ob ein punk um einen room ist
//-> damit pfadsuche nicht durch raum geht
//liefert none, wenn es nicht near ist, aber die jeweilige direction in der room ist
//
pub fn is_cord_near_room(cords: Cords, room: &Room) -> Option<Direction> {
    if
        cords.x > (room.cords.x) &&
        cords.x < (room.cords.x + room.width - 1) &&
        cords.y > (room.cords.y) &&
        cords.y < (room.cords.y + room.height - 1) {

        if cords.x <= room.cords.x { return Some(Direction::EAST); }
        else if cords.x >= (room.cords.x + room.width -1) { return Some(Direction::WEST); }
        else if cords.y <= room.cords.y { return Some(Direction::NORTH); }
        else if cords.y >= (room.cords.y + room.height -1) { return Some(Direction::SOUTH); }
        return Some(Direction::NORTH);
    }
    None
}
pub fn set_room_tile(local_x: i32, local_y: i32, room: &Room) -> Tile {

        let left   = room.cords.x;
        let right  = room.cords.x + room.width - 1;
        let top    = room.cords.y + room.height - 1;
        let bottom = room.cords.y;

        if local_x > left && local_x < right && local_y < top && local_y > bottom {
            return Tile::Floor;
        }

        let on_vertical   = (local_x == left || local_x == right)
                         && local_y >= bottom && local_y <= top;
        let on_horizontal = (local_y == top || local_y == bottom)
                         && local_x >= left && local_x <= right;

        if !on_vertical && !on_horizontal {
            return Tile::Wall;
        }

        // Ecken sind immer Wand
        let is_corner = (local_x == left || local_x == right)
                     && (local_y == top  || local_y == bottom);
        if is_corner {
            return Tile::Wall;
        }

        // Bestimme, auf welcher Seite die Tile liegt
        let side = if local_y == top    { Direction::NORTH }
                   else if local_y == bottom { Direction::SOUTH }
                   else if local_x == left   { Direction::WEST  }
                   else                      { Direction::EAST  };

        let temp_cords = Cords::new(local_x, local_y);
        if temp_cords == room.entry_cords { return Tile::Floor; }
        return Tile::Wall;

    Tile::Wall
}

//calc cordinates of entry of a room
pub fn entry_cords_from_direction_and_type(
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

//set direction by a seed (used for room setting entry)
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

    a.cords.x - padding < b.cords.x + b.width
        && a.cords.x + a.width + padding > b.cords.x
        && a.cords.y - padding < b.cords.y + b.height
        && a.cords.y + a.height + padding > b.cords.y
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
