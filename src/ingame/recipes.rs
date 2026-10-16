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

impl Recipe {

    //none, wenn nicht craftable, ansonsten anzahl, wie oft craftable
    pub fn is_craftable(&self, player_items: HashMap<ItemId, u32>) -> Option<u32> {
        let mut counts: Vec<u32> = Vec::new();

        for recipe_item in self.items.clone() {
            match player_items.get(&recipe_item.id) {
                Some(count) => {
                    if *count < recipe_item.count { return None; }
                    let num_2_craft = *count / recipe_item.count;
                    counts.push(num_2_craft);
                }
                None => { return None; }
            }
        }
        counts.into_iter().map(|c| c * self.result.count).min() //diese funktion is so rust core
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
    let mut craftable_recipes: Vec<ItemStack> = Vec::new();

    for recipe in recipes.0.clone() {
        match recipe.is_craftable(player_items.clone()) {
            Some(count) => { craftable_recipes.push(ItemStack { id: recipe.result.id, count: count}); }
            None => { continue; }
        }
    }

    println!("recipe craftable: \n{:?}", craftable_recipes);

    craftable_items.0.clear();
    craftable_items.0 = craftable_recipes;

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
fn merge_stack(items: Vec<ItemStack>) -> HashMap<ItemId, u32> {
    let mut map: HashMap<ItemId, u32> = HashMap::new();

    for item in items {
        *map.entry(item.id).or_insert(0) += item.count;
    }
    return map
}
