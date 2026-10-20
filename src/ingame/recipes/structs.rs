use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::ingame::items::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub items: Vec<ItemStack>, //item + anzahl des items
    pub station: Option<CraftStation>,
    pub result: ItemStack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CraftStation {
    Workbench,
}

#[derive(Resource)]
pub struct Recipes(pub Vec<Recipe>);


#[derive(Resource)]
pub struct CraftableItems(pub Vec<ItemStack>);

//tooltip im craftui
#[derive(Component)]
pub struct CursorCraftInfoText;

#[derive(Component)]
pub struct CraftableList;          // eindeutiger Marker für den Container

#[derive(Component)]
pub struct CraftableSlot(pub usize); // index in CraftableItems.0

#[derive(Component)]
pub struct CraftableCountText;

#[derive(Component)]
pub struct CraftableTooltip;