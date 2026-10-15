use bevy::prelude::*;
use bevy::sprite::*;
use crate::ingame::inventory::*;
use crate::ingame::inventory_meta::*;

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

pub fn mana_regen(mut player: Query<&mut Player>) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    if player.mana == player.max_mana {
        return;
    }
    let mana_regen_num = 2.0;
    if (player.mana + mana_regen_num) >= player.max_mana {
        player.mana = player.max_mana;
        return;
    }

    player.mana += mana_regen_num;
}

pub fn health_regen(mut player: Query<&mut Player>) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    if player.health == player.max_health {
        return;
    }
    let health_regen_num = 2.0;
    if (player.health + health_regen_num) >= player.max_health {
        player.health = player.max_health;
        return;
    }

    player.health += health_regen_num;
}
