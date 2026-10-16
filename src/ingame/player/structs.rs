use bevy::prelude::*;
use bevy::sprite::*;
use crate::ingame::inventory::util::*;
use crate::ingame::inventory::systems::*;
use crate::ingame::inventory::ui::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::player::ui::*;
use crate::ingame::player::util::*;
use crate::ingame::player::systems::*;
use crate::ingame::util::*;
use crate::ingame::util::*;
use crate::ingame::systems::*;
use crate::ingame::ui::*;

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