use bevy::prelude::*;
use bevy::sprite::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::world::seed_service::*;
use crate::world::region::*;

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

//nimm alle räume und erstelle eine liste an koordinaten
//-> gemalte koordinaten sind pfad
pub fn generate_room_paths(rooms: &Vec<Room>, seed: u64) -> Vec<Cords> {
    let mut paths: Vec<Cords> = Vec::new();
    for (index, room) in rooms.iter().enumerate() {
        let next_room = match rooms.get(index + 1) {
            Some(room) => room,
            None => { break; },
        };
        paths.append(&mut depth_search((room, next_room)));
    }

    paths
}

//suchalgo implementiert
//tiefensuch, für schöne lange pfade
//regeln zu implementieren:
//  - keine max eine richtungsänderung alle zwei tiles
//  - immer mind. einen tile abstand (wall) zwischen wegen
//      -> dafür evtl bereits besuchte in liste speichern und hier übergeben
pub fn depth_search(rooms: (&Room, &Room)) -> Vec<Cords> {
    let c1 = Cords {
        x: 1,
        y:1,
    };
    let c2 = Cords {
        x: 2,
        y:3,
    };
    let c3 = Cords {
        x: 3,
        y:2,
    };
    return vec![c1, c2, c3];
}

//checke, ob ein punk um einen room ist
//-> damit pfadsuche nicht durch raum geht
pub fn is_cord_near_room(cords: Cords, room: Room) -> bool {
    false
}

pub fn set_room_tile(local_x: i32, local_y: i32, room: &Room) -> Tile {

        let left   = room.x;
        let right  = room.x + room.width - 1;
        let top    = room.y + room.height - 1;
        let bottom = room.y;

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
