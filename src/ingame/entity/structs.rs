use bevy::prelude::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::modifier::*;
use crate::ingame::items::attack::structs::*;
use crate::constants::*;

#[derive(Component)]
#[require(Velocity)]
pub struct WallCollidable;

#[derive(Component)]
pub struct MyEntity;

#[derive(Component)]
pub struct Health {
    pub health_points: f32,
    pub max_health: f32, //normal: 100
    pub health_regen: f32, //normal: 1/s
}

#[derive(Component)]
pub struct Mana {
    pub mana_points: f32, 
    pub max_mana: f32, //normal: 100
    pub mana_regen: f32, //normal: 1/s
}

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity, Stats, Health, Mana, Equipment, Attacker, Hitbox, Hittable)]
pub struct Enemy {
    pub weapon: ItemId,
    pub speed: f32,
    pub stop_distance: f32,
}

#[derive(Component)]
#[require(Transform)]
pub struct EnemyHealthBar;

#[derive(Component)]
#[require(Transform)]
pub struct EnemyHealthBarBackground;



impl Enemy {
    pub fn new() -> Self {
        Self {
            weapon: ItemId::WoddenDagger,
            speed: 100.0,
            stop_distance: 4.0,
        }
    }
}


//hitbox

#[derive(Component, Default)]
pub struct Hittable;

/// Körperbox relativ zur Root-Position.
/// `offset` = untere linke Ecke der Box relativ zu transform.translation
#[derive(Component, Clone, Copy, Debug)]
pub struct Hitbox {
    pub size: Vec2,
    pub offset: Vec2, // untere linke Ecke relativ zu translation
}

impl Default for Hitbox {
    fn default() -> Self {
        let size = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
        Self { size, offset: -size * 0.5 } // zentriert um translation
    }
}

impl Hitbox {
    pub fn min(&self, pos: Vec2) -> Vec2 { pos + self.offset }
    pub fn max(&self, pos: Vec2) -> Vec2 { pos + self.offset + self.size }
    pub fn center(&self, pos: Vec2) -> Vec2 { pos + self.offset + self.size * 0.5 }
    pub fn aabb(&self, pos: Vec2) -> Aabb2d {
        Aabb2d { min: self.min(pos), max: self.max(pos) }
    }

    pub fn draw(&self, giz: &mut Gizmos, tf: Transform) {
        if !DRAW_HITBOX { return; }
        
        // Mittelpunkt der Box relativ zum Entity, dann mit Rotation/Skalierung in die Welt
        let local_center = (self.offset + self.size * 0.5).extend(0.0);
        let world_center = tf.transform_point(local_center).truncate();
        let angle = tf.rotation.to_euler(EulerRot::ZYX).0;

        giz.rect_2d(
            Isometry2d::new(world_center, Rot2::radians(angle)),
            self.size * tf.scale.truncate(),
            Color::srgb(1.0, 0.0, 0.0),
        );
    }

}

