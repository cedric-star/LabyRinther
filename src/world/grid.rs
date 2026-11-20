use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use crate::ingame::entity::structs::*;

pub trait Grid {
    //eingabe tilekoordinate -> tile? (floor, wall, ...)
    fn tile(&self, t: IVec2) -> Option<Tile>;

    fn block_sight(&self, t: IVec2) -> bool;

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

    fn block_sight(&self, t: IVec2) -> bool {
        match self.tile(t) {
            Some(Tile::Wall) | None => true,
            Some(_) => false,
        }
    }


    fn can_view(&self, viewer: &Enemy, viewer_pos: &Transform, to_view: &Transform) -> bool {
        let dir_vec = to_view.translation.truncate() - viewer_pos.translation.truncate();

        let angle = dir_vec.normalize_or_zero().to_angle().to_degrees(); //in grad
        if Direction::from_angle_deg(angle) != viewer.heading { return false; }

        let from_tile = viewer_pos
            .translation
            .truncate()
            .floor()
            .as_ivec2()
            .world_to_tile();
        let to = to_view
            .translation 
            .truncate()
            .floor()
            .as_ivec2() 
            .world_to_tile();

        let mut cur = from_tile;

        // Bresenham-Vorbereitung
        let d = (to - from_tile).abs();                 // Abstand in Tiles pro Achse
        let step = IVec2::new(                          // Schrittrichtung: +1 oder -1
            if from_tile.x < to.x { 1 } else { -1 },
            if from_tile.y < to.y { 1 } else { -1 },
        );
        let mut err = d.x - d.y;                        // Fehlerterm: Abweichung von der idealen Geraden

        while cur != to {
            let e2 = 2 * err;
            // Je nach Fehlerterm in x, in y oder in beide Richtungen einen Schritt gehen
            if e2 > -d.y { err -= d.y; cur.x += step.x; }
            if e2 <  d.x { err += d.x; cur.y += step.y; }

            // Zwischenfelder auf Wand prüfen; das Zielfeld selbst nicht
            if cur != to && self.block_sight(cur) { return false; }
        }

        true
    }
}