use bevy::prelude::*;
use crate::constants::*;
use crate::ingame::inventory::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ingame::items::*;
use std::fs;
use crate::components::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::ui::*;
use crate::ingame::player::util::*;
use crate::ingame::player::systems::*;
use crate::ingame::player::structs::*;
use crate::ingame::ui::*;
use crate::util::ui::constants::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::systems::*;
use crate::ingame::inventory::ui::*;
use crate::ingame::recipes::systems::*;
use crate::ingame::recipes::ui::*;
use crate::ingame::recipes::util::*;
use crate::ingame::util::*;
use crate::ingame::util::*;
use crate::ingame::systems::*;
use crate::ingame::ui::*;

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

#[derive(Component)]
pub struct CraftableList;          // eindeutiger Marker für den Container

#[derive(Component)]
pub struct CraftableSlot(pub usize); // index in CraftableItems.0

#[derive(Component)]
pub struct CraftableCountText;

#[derive(Component)]
pub struct CraftableTooltip;