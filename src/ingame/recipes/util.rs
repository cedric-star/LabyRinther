use bevy::prelude::*;
use std::collections::HashMap;
use crate::ingame::items::*;
use std::fs;
use crate::ingame::inventory::structs::*;
use crate::ingame::recipes::structs::*;

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


impl Default for CraftableItems {
    fn default() -> Self {
        Self(Vec::new())
    }
}

pub fn merge_stack(items: Vec<ItemStack>) -> HashMap<ItemId, u32> {
    let mut map: HashMap<ItemId, u32> = HashMap::new();

    for item in items {
        *map.entry(item.id).or_insert(0) += item.count;
    }
    return map
}

// zieht `amount` items des typs aus hotbar + inneren reihen
pub fn remove_items(inv: &mut Inventory, id: ItemId, mut amount: u32) {
    let mut take_from = |slot: &mut InventorySlot| {
        if amount == 0 { return; }
        if let Some(s) = &mut slot.stack {
            if s.id == id {
                let take = s.count.min(amount);
                s.count -= take;
                amount -= take;
                if s.count == 0 { slot.stack = None; }
            }
        }
    };
    for slot in inv.hotbar.iter_mut() { take_from(slot); }
    for row in inv.inner_rows.iter_mut() {
        for slot in row.iter_mut() { take_from(slot); }
    }
}

pub fn count_items(inv: &Inventory, id: ItemId) -> u32 {
    inv.hotbar
        .iter()
        .chain(inv.inner_rows.iter().flatten())
        .filter_map(|slot| slot.stack)
        .filter(|s| s.id == id)
        .map(|s| s.count)
        .sum()
}
