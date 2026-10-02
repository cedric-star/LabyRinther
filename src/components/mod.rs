use bevy::prelude::*;
use crate::constants::*;
use std::collections::HashSet;
use indexmap::IndexSet;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity)]
pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct Collider {
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Chunk {
    pub tiles: [[Tile; CHUNK_SIZE]; CHUNK_SIZE],
    pub cords: Cords,
}

#[derive(Clone, Hash, Eq, Copy, Debug, PartialEq)]
pub struct Cords {
    pub x: i32,
    pub y: i32,
}

impl Cords {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    pub fn to_world(self) -> Vec2 {
        Vec2 {
            x: (self.x as f32) * TILE_SIZE + TILE_SIZE,
            y: (self.y as f32) * TILE_SIZE + TILE_SIZE,
        }

    }

    pub fn from_world(world_pos: Vec2) -> Self {
        Self {
            x: (world_pos.x / TILE_SIZE).floor() as i32,
            y: (world_pos.y / TILE_SIZE).floor() as i32,
        }
    }

    pub fn chunk_to_world(self) -> Vec2 {
        Vec2 {
            x: (self.x as f32) * (CHUNK_SIZE as f32) * TILE_SIZE,
            y: (self.y as f32) * (CHUNK_SIZE as f32) * TILE_SIZE,
        }
    }

    pub fn world_to_chunk(world_pos: Vec2) -> Self {
        Self {
            x: (world_pos.x / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32,
            y: (world_pos.y / (CHUNK_SIZE as f32 * TILE_SIZE)).floor() as i32,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Region {
    pub cords: Cords,
    pub rooms: IndexSet<Room>,                    // war: Vec<Room>
    pub paths: IndexSet<Cords>,
    pub connection_tiles: IndexSet<(Direction, Cords)>, // ggf. IndexSet, s.u.
}

#[derive(Component,Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Room {
    pub id: RoomId,

    pub cords: Cords,
    pub width: i32,
    pub height: i32,

    pub entry: Direction,
    pub entry_cords: Cords,
    pub room_type: RoomType,
}
#[derive(Component,Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Component,Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum RoomType {
    SMALL,
    NORMAL,
    LARGE,
}

impl RoomType {
    pub fn dimensions(self) -> (i32, i32) {
        match self {
            RoomType::SMALL => (8, 6),
            RoomType::NORMAL => (12, 7),
            RoomType::LARGE => (20, 12),
        }
    }
}

pub struct Connection {
    pub from: RoomId,
    pub to: RoomId,

    pub kind: ConnectionKind,
}

pub enum ConnectionKind {
    Corridor,
    Maze,
}

#[derive(Component,Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct RoomId(pub i32);
