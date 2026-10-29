use bevy::prelude::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::modifier::*;
use crate::ingame::items::attack::*;


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

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity, Stats, Health, Mana, Equipment, Attacker)]
pub struct Enemy {
    pub weapon: ItemId,
}



impl Enemy {
    pub fn new() -> Self {
        Self {
            weapon: ItemId::WoddenDagger,
        }
    }
}




