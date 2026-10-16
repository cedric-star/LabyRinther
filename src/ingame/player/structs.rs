use bevy::prelude::*;
use crate::ingame::inventory::structs::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity)]
pub struct Player {
    pub inventory: Inventory,
    pub health: f32,
    pub max_health: f32, //normal: 100
    pub armor: f32, //0-60
    pub mana: f32,
    pub max_mana: f32,
    pub speed: f32,

}