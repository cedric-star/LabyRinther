use bevy::prelude::*;


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
