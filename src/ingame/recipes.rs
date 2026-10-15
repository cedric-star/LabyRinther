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

#[derive(Resource)]
pub struct CraftableItems(Vec<ItemStack>);

impl Default for CraftableItems {
    fn default() -> Self {
        Self(Vec::new())
    }
}

//liste enthält alle craftable items, und wie viele jeweils
pub fn set_craftable_items(
    player: Query<&Player>,
    mut craftable_items: ResMut<CraftableItems>,
    recipes: Res<Recipes>,
) {
    let Ok(player) = player.single() else {
        panic!("didnt found player coreclty");
    };

    let mut player_items: Vec<ItemStack> = Vec::new();
    for inv_rows in player.inventory.inner_rows.clone() {
        for slot in inv_rows {
            if let Some(slot_unwrapped) = slot.stack {
                player_items.push(slot_unwrapped);
            }
        }
    }
    for slot in player.inventory.hotbar {
        if let Some(slot_unwrapped) = slot.stack {
            player_items.push(slot_unwrapped);
        }
    }

    let player_items = merge_stack(player_items);

    //alle craftbaren recipes:
    let craftable_recipes: Vec<Recipe> = Vec::new();

    for recipe in recipes.0.clone() {
        println!("recip: {:?}", recipe);
    }

    return;
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

//helper
//
fn merge_stack(items: Vec<ItemStack>) -> Vec<ItemStack> {
    let mut map: HashMap<ItemId, u32> = HashMap::new();

    for item in items {
        *map.entry(item.id).or_insert(0) += item.count;
    }

    map.into_iter()
        .map(|(id, count)| ItemStack { id, count })
        .collect()
}
