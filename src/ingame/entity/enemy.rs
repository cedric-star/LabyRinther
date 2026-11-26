use bevy::prelude::*;
use rand::prelude::*;
use avian2d::prelude::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::modifier::*;
use crate::ingame::items::attack::structs::*;
use crate::components::*;
use crate::ingame::entity::structs::*;

#[derive(Component)]
#[require(Transform, Velocity, Stats, Health, Mana, Equipment, Attacker, Hittable, CollidingEntities)]
pub struct Enemy {
    pub weapon: ItemId,
    pub heading: Direction,
    pub target: EnemyTarget,
    pub stop_distance: f32,
}

#[derive(Component, PartialEq)]
pub enum EnemyTarget {
    NextPlayer,
    Nothing(MovementState),//einfach irgendeine random richtung
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
            target: EnemyTarget::Nothing(MovementState::Standing(
                StandingMovementState { current: 0.0, max: 2.0 }
            )),
            stop_distance: 4.0,
            heading: Direction::North,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MovementState {
    Standing(StandingMovementState),
    Walking(WalkingMovementState),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct StandingMovementState {
    pub current: f32, //how long standing
    pub max: f32,//how long to stand
}

impl Default for StandingMovementState {
    fn default() -> Self {
        Self {
            current: 0.0,
            max: rand::random_range(1.0..=6.0),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct WalkingMovementState {
    pub current: f32,//how long is waling
    pub max: f32,  //how long should be walking
    pub direction: Vec2, //laufrichtung
}

impl Default for WalkingMovementState {
    fn default() -> Self {
        let random_radians = rand::random_range(0.0..std::f32::consts::TAU); //tau == 2*pi == kreis
        Self {
            current: 0.0,
            max: rand::random_range(1.0..=3.0),
            direction: Vec2::from_angle(random_radians),
        }
    }
}
