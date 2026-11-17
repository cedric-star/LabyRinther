use std::default;

use bevy::prelude::*;
use avian2d::prelude::*;


#[derive(Component)]
pub struct IngameUi;

#[derive(Component)]
pub struct HotBar;

#[derive(Component)]
pub struct HotBarSelector;

#[derive(Component, Debug)]
pub struct HotBarItem(pub usize);

#[derive(Component)]
pub struct HotBarItemNum(pub usize);

#[derive(PhysicsLayer, Default)]
pub enum Layer {
    #[default]
    Default,
    Wall, 
    Player,
    Enemy,
    PlayerWeapon,
    EnemyWeapon,
}