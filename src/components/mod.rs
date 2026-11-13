use bevy::prelude::*;
use bevy::math::*;
use crate::constants::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Welcome,
    InGame,
    Settings,
}

#[derive(Resource, Default, Debug)]
pub struct LastState(pub Option<GameState>);

#[derive(Component)]
pub struct GameEntity;


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
    RoomFloor,
    RoomEntry,
}

impl Tile {
    pub fn asset(self) -> &'static str {
        match self {
            Tile::Wall => "wall_simple.png",
            Tile::Floor => "ground_simple.png",
            Tile::RoomFloor => "room_ground_simple.png", // später eigene Textur
            Tile::RoomEntry => "ground_simple.png",     // später eigene Textur
        }
    }
}

#[derive(Component, Debug)]
pub struct Chunk {
    pub tiles: [[Tile; U_CHUNK_SIZE]; U_CHUNK_SIZE],
    pub cords: IVec2,
}

pub trait CordsExt {
    fn world_to_tile(self) -> IVec2; //gibt tile cords zurück in der der pixel liegt
    fn world_to_chunk(self) -> IVec2; //chunk cord, in dem eingabe world pixel liegen
    fn world_to_region(self) -> IVec2; //region cord, in der world pixel cords liegt

    fn tile_to_world(self) -> IVec2; //world kord, des unteren linken pixels des tiles
    fn tile_to_chunk(self) -> IVec2; //chunk cord, in dem die tile liegt
    fn tile_to_region(self) -> IVec2; //region cords, in der die tile liegt

    fn chunk_to_world(self)-> IVec2; //unterer linker px des chunks
    fn chunk_to_tile(self) -> IVec2; //unterer linker tile eines chunks
    fn chunk_to_region(self) -> IVec2; //region cords, in der der chunk liegt

    fn region_to_world(self) -> IVec2; //unterer linker pixel der region
    fn region_to_tile(self) -> IVec2; //unterer linker tile der region
    fn region_to_chunk(self) -> IVec2; //unterer linker chunk in einer region
}

impl CordsExt for IVec2 {
    fn world_to_tile(self) -> IVec2 { self.div_euclid(TILE_SIZE_VEC2) }
    fn world_to_chunk(self) -> IVec2 { self.div_euclid(CHUNK_WORLD_SIZE_VEC2) }
    fn world_to_region(self) -> IVec2 { self.div_euclid(REGION_WORLD_SIZE_VEC2) }

    fn tile_to_world(self) -> IVec2 { self * TILE_SIZE_VEC2 }
    fn tile_to_chunk(self) -> IVec2 { self.div_euclid(CHUNK_SIZE_VEC2) }
    fn tile_to_region(self) -> IVec2 { self.div_euclid(REGION_TILE_SIZE_VEC2) }

    fn chunk_to_world(self)-> IVec2 { self * CHUNK_WORLD_SIZE }
    fn chunk_to_tile(self) -> IVec2 { self * CHUNK_SIZE }
    fn chunk_to_region(self) -> IVec2 { self.div_euclid(REGION_SIZE_VEC2) }

    fn region_to_world(self) -> IVec2 { self * REGION_WORLD_SIZE }
    fn region_to_tile(self) -> IVec2 { self * REGION_TILE_SIZE }
    fn region_to_chunk(self) -> IVec2 { self * REGION_SIZE }
}

#[derive(Component, Clone, Debug)]
pub struct Region {
    pub cords: IVec2,
    pub rooms: Vec<Room>,
    pub map: crate::world::region::RegionMap,
}

#[derive(Component,Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Room {
    pub id: RoomId,

    pub cords: IVec2,
    pub width: i32,
    pub height: i32,

    pub entry: Direction,
    pub entry_cords: IVec2,
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
    pub fn cells(self) -> (usize, usize) {
        match self {
            RoomType::SMALL => (2, 2),
            RoomType::NORMAL => (3, 2),
            RoomType::LARGE => (5, 3),
        }
    }

    pub fn dimensions(self) -> (i32, i32) {
        let (w, h) = self.cells();
        (w as i32 * CELL_PITCH - 1, h as i32 * CELL_PITCH - 1)
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
