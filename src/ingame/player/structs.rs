use bevy::prelude::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::modifier::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity, Stats, Health, Mana, Inventory, Equipment)]
pub struct Player;


