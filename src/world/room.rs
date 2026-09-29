use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::world::seed_service::*;
use crate::world::region::*;
use std::collections::{VecDeque, HashMap, HashSet};

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
        paths.append(&mut depth_search((room, next_room), &rooms, seed));
    }

    paths
}

pub fn depth_search(rooms: (&Room, &Room), all_rooms: &Vec<Room>, seed: u64) -> Vec<Cords> {
    let mut visited: Vec<Cords> = Vec::new();
    visited.push(rooms.0.entry_cords);
    let target = rooms.1.entry_cords;

    println!("{:?} -> {:?}", rooms.0.entry_cords, target);

    while true {
        let last = visited.last().unwrap();
        if last == &target {
            return visited;
        }

        let new_x =
        if last.x < target.x { last.x + 1 }
        else if last.x > target.x { last.x - 1 }
        else { last.x };

        let new_y =
        if last.y < target.y { last.y + 1 }
        else if last.y > target.y { last.y - 1 }
        else { last.y };

        let mut new_cords = Cords {
            x: new_x,
            y: new_y,
        };

        for my_room in all_rooms.iter() {
            match is_cord_near_room(new_cords, my_room) {
                None => { },
                Some(Direction::NORTH) => { new_cords.y -= 1; },
                Some(Direction::SOUTH) => { new_cords.y += 1; },
                Some(Direction::EAST) => { new_cords.x -= 1; },
                Some(Direction::WEST) => { new_cords.x += 1; },
            }
        }
        visited.push(new_cords);
    }

    return Vec::new();
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
