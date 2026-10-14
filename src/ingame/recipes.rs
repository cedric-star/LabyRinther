use bevy::prelude::*;
use crate::constants::*;
use crate::ingame::inventory::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ingame::inventory_meta::*;
use crate::ingame::items::*;
use std::fs;
use crate::components::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::*;
use crate::ingame::ui::*;
use crate::util::ui::constants::*;
use crate::ingame::inventory::*;

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
pub struct Recipes(Vec<Recipe>);

impl Default for Recipes {
    fn default() -> Self {
        let path = "./recipes.json";
        let file = match fs::read_to_string(path) {
            Ok(file) => {
                if file.trim().is_empty() { panic!("recipesjson emtpy):"); }
                file
            }
            Err(e) => { panic!("recipes json nicht ladbar :()"); }
        };

        match serde_json::from_str::<Vec<Recipe>>(&file) {
            Ok(list) => Self(list),
            Err(e) => {panic!("json recipes ungültig )):::")}
        }
    }
}

pub fn get_craftable_item_field(
    s: &GameSettings,
    player: &Player
) -> impl Bundle {
    let scale = s.ui_scale;

    (
        IngameUi,
        GameEntity,
        InventoryShow,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            grid_column: GridPlacement::start(1),
            grid_row: GridPlacement::start(2),
            ..default()
        },
        Visibility::Hidden,
        BackgroundColor(BG_COL_A),
    )
}
