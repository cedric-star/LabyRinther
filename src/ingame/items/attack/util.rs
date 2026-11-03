use bevy::prelude::*;
use bevy::math::bounding::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::attack::structs::*;

impl Default for Attacker {
    fn default() -> Self {
        Self {
            active: None,
            cooldown_left: 0.0,               
        }
    }
}

/// Vier Weltecken der Sprite-Hitbox (Rotation, Position, Skalierung aus GlobalTransform)
pub fn world_corners(hitbox: &Hitbox, gt: &GlobalTransform) -> [Vec2; 4] {
    let xx = hitbox.offset;
    let yy = hitbox.offset + hitbox.size;
    [xx, Vec2::new(yy.x, xx.y), yy, Vec2::new(xx.x, yy.y)]
        .map(|c| gt.transform_point(c.extend(0.0)).truncate())
}

pub fn project(pts: &[Vec2; 4], axis: Vec2) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for p in pts {
        let d = p.dot(axis);   // Projektion des Punkts auf die Achse
        min = min.min(d);
        max = max.max(d);
    }
    (min, max)
}
pub fn obb_hits_aabb(c: &[Vec2; 4], aabb: &Aabb2d) -> bool {
    let e1 = (c[1] - c[0]).normalize_or_zero(); // Kantenrichtung 1 der Waffe
    let e2 = (c[3] - c[0]).normalize_or_zero(); // Kantenrichtung 2 der Waffe
    let b = [
        aabb.min,
        Vec2::new(aabb.max.x, aabb.min.y),
        aabb.max,
        Vec2::new(aabb.min.x, aabb.max.y),
    ];
    for axis in [Vec2::X, Vec2::Y, e1, e2] {
        let (a_min, a_max) = project(c, axis);
        let (b_min, b_max) = project(&b, axis);
        if a_max < b_min || b_max < a_min {
            return false; // Lücke gefunden -> keine Überschneidung
        }
    }
    true
}