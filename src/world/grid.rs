use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::ingame::entity::structs::*;

pub trait Grid {
    //eingabe tilekoordinate -> tile? (floor, wall, ...)
    fn tile(&self, t: IVec2) -> Option<Tile>;

    fn can_view(&self, viewer: &Enemy, viewer_pos: &Transform, to_view: &Transform) -> bool;

}

#[derive(SystemParam)]
pub struct WorldGrid<'w, 's> {
    pool: Res<'w, ChunkPool>,
    chunks: Query<'w, 's, &'static Chunk>,
}

impl Grid for WorldGrid<'_, '_> {
    fn tile(&self, t: IVec2) -> Option<Tile> {
        let chunk_cords = t.tile_to_chunk(); //chunk koordinaten, wo tile liegt
        let entity = self.pool.chunks.get(&chunk_cords)?; // entity aus hashmap holen (bz. aus pool)
        let chunk = self.chunks.get(*entity).ok()?; //query -> entity(id) -> &Chunk
        let local_tile_in_chunk = t.rem_euclid(CHUNK_SIZE_VEC2); //lokale position im chunk von tile

        Some(chunk.tiles[local_tile_in_chunk.x as usize][local_tile_in_chunk.y as usize])
    }

    fn can_view(&self, viewer: &Enemy, viewer_pos: &Transform, to_view: &Transform) -> bool {
        let dir_vec = to_view.translation.truncate() - viewer_pos.translation.truncate();

        let angle = dir_vec.normalize().to_angle().to_degrees(); //in grad
        if Direction::from_angle_deg(angle) != viewer.heading { return false; }
        

        false
    }
}