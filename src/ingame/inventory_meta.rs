use bevy::prelude::*;
use crate::constants::*;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use crate::ingame::items::*;

#[derive(Debug, Component)]
pub struct Inventory {
    pub hotbar: [InventorySlot; 10],
    pub selected: usize,
    pub inner_rows: Vec<[InventorySlot; 10]>,
    pub equipment: Equipment,
}

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub helm: Option<ItemStack>,
    pub chest: Option<ItemStack>,
    pub legs: Option<ItemStack>,
    pub boots: Option<ItemStack>,
    pub usable: Option<ItemStack>,
    pub secret: Option<ItemStack>,
}

#[derive(Clone, Debug, Default, Copy)]
pub struct InventorySlot {
    pub stack: Option<ItemStack>,
}

pub const MAX_INNER_ROWS: usize = 6;
pub const HOTBAR_ROW: usize = MAX_INNER_ROWS;

impl Inventory {
    pub fn row_offset(&self) -> usize { //leere reihen, oben im ui
        MAX_INNER_ROWS.saturating_sub(self.inner_rows.len()) //overflow verhindern, 0
        //return
    }

    //get inv slot by cords
    pub fn slot(&self, x: usize, y: usize) -> Option<&InventorySlot> {
        if y == HOTBAR_ROW {
            return self.hotbar.get(x);
        }
        let r = y.checked_sub(self.row_offset())?; //returnt none bei over/underflow
        self.inner_rows.get(r)?.get(x)
    }

    pub fn slot_mut(&mut self, x: usize, y: usize) -> Option<&mut InventorySlot> {
        if y == HOTBAR_ROW {
            return self.hotbar.get_mut(x);
        }
        let r = y.checked_sub(self.row_offset())?;
        self.inner_rows.get_mut(r)?.get_mut(x)
    }

    pub fn equip_mut(&mut self, s: EquipSlot) -> &mut Option<ItemStack> {
        match s {
            EquipSlot::Helm     => &mut self.equipment.helm,
            EquipSlot::Chest    => &mut self.equipment.chest,
            EquipSlot::Legs     => &mut self.equipment.legs,
            EquipSlot::Boots    => &mut self.equipment.boots,
            EquipSlot::Usable   => &mut self.equipment.usable,
            EquipSlot::Secret   => &mut self.equipment.secret,
        }
    }

}

impl Default for Inventory {
    fn default() -> Self {

        let es = InventorySlot { stack: None, };
        let empty_row = [es; 10];
        let mut other_rows = Vec::new();
        other_rows.push(empty_row);
        other_rows.push(empty_row);

        Self {
            hotbar: [
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::WoddenDagger,
                    count: 1,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::Apple,
                    count: 4,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::IronChest,
                    count: 1,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::WoddenDagger,
                    count: 1,
                })},
                es.clone(),
                es.clone(),
                es.clone(),
                es.clone(),
                es.clone(),
                es.clone(),
            ],
            selected: 0,
            equipment: Equipment::default(),
            inner_rows: other_rows,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)] //nur manche items in inventar-slots
pub enum EquipSlot { Helm, Chest, Legs, Boots, Usable, Secret }

impl EquipSlot {
    pub fn accepts(self, id: ItemId, items: &Items) -> bool {
        let cats = items.0.get(&id).unwrap().categories.clone();
        match self {
            EquipSlot::Helm     => cats.contains(&Category::Helm),
            EquipSlot::Chest    => cats.contains(&Category::Chest),
            EquipSlot::Legs     => cats.contains(&Category::Legs),
            EquipSlot::Boots    => cats.contains(&Category::Boots),
            EquipSlot::Usable   => cats.contains(&Category::Usable),
            EquipSlot::Secret   => cats.contains(&Category::Secret),
        }
    }
}

pub fn can_place(restriction: Option<EquipSlot>, id: ItemId, items: &Items) -> bool {
    restriction.map_or(true, |s| s.accepts(id, items))
}
