use bevy::prelude::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::modifier::*;
use crate::ingame::items::attack::structs::*;
use crate::components::*;

#[derive(Component)]
pub struct MyEntity;

#[derive(Component)]
pub struct Health {
    pub health_points: f32,
    pub max_health: f32, //normal: 100
}

#[derive(Component)]
pub struct Mana {
    pub mana_points: f32, 
    pub max_mana: f32, //normal: 100
}

#[derive(Component, Default, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct Hittable;