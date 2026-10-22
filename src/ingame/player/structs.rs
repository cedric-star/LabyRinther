use bevy::prelude::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::entity::structs::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity)]
pub struct Player {
    pub inventory: Inventory,
    pub health: Health,
    pub armor: f32, //0-60
    pub mana: Mana,
    pub speed: f32,

}