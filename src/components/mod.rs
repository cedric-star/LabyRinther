use bevy::prelude::*;
use crate::{constants::*, ingame::entity::structs::EnemyHealthBar};

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


pub enum ZLayer {
    Floor,
    Wall, 
    Enemy,
    EnemyHealthBarBackground,
    EnemyHealthBarForeground,
    Player,
    Weapon,
    Projectile,
    Ui,
    UiLevatating,
    Camera,
}

impl ZLayer {
    pub const fn z(self) -> f32 {
        use ZLayer::*;
        match self {
            Floor => 0.0,
            Wall => 5.0,
            Enemy => 10.0,
            EnemyHealthBarBackground => 11.0,
            EnemyHealthBarForeground => 12.0,
            Player => 15.0,
            Weapon => 20.0,
            Projectile => 25.0,
            Ui => 30.0,
            UiLevatating => 35.0,
            Camera => 100.0,
        }
    }
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
    pub fn get_z_layer(&self) -> ZLayer {
        match self {
            Tile::Wall => ZLayer::Wall,
            Tile::Floor | Tile::RoomFloor | Tile::RoomEntry => ZLayer::Floor,
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

    //funky extra funcs
    fn transform_to_tile(tr: &Transform) -> Self; //nimmt eine world pos und berechnet tile koordinate
    fn get_surrounding(&self, size: i32) -> Vec<IVec2>;
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

    fn transform_to_tile(tr: &Transform) -> Self {
        tr.translation 
            .truncate()
            .floor()
            .as_ivec2() 
            .world_to_tile()
    }

    //unabhängig von tile oder chunk, auf alle anwendbar, da mit ganzzahligen zahlen gerechnet wird.
    fn get_surrounding(&self, size: i32) -> Vec<IVec2> {
        let mut my_vec: Vec<IVec2> = Vec::new();

        for x in -size..=size { for y in -size..=size {
            let mut new_cord = *self;
            new_cord.x = self.x + x;
            new_cord.y = self.y + y;
            my_vec.push(new_cord);
        }}

        my_vec
    }
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
    North,
    East,
    South,
    West,
}

impl Direction {
    /// angle in Grad, wie von Vec2::to_angle().to_degrees() geliefert:
    /// 0° = Ost (+X), 90° = Nord (+Y), gegen den Uhrzeigersinn
    pub fn from_angle_deg(angle: f32) -> Direction {
        let angle = angle.rem_euclid(360.0); // -30° -> 330°, alles in 0..360

        match angle {
            a if a <  45.0 => Direction::East,
            a if a < 135.0 => Direction::North,
            a if a < 225.0 => Direction::West,
            a if a < 315.0 => Direction::South,
            _              => Direction::East, // 315..360 gehört wieder zu Ost
        }
    }

    //alternative: from vector2d
    pub fn from_vec(v: Vec2) -> Direction {
        // Die dominante Achse entscheidet
        if v.x.abs() > v.y.abs() {
            if v.x > 0.0 { Direction::East } else { Direction::West }
        } else if v.y > 0.0 {
            Direction::North
        } else {
            Direction::South
        }
    }
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


#[derive(Component,Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct RoomId(pub i32);
