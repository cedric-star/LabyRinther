use bevy::prelude::*;
use crate::constants::*;
use std::collections::HashSet;


#[derive(Default, Debug, Component)]
pub struct Inventory {
    pub hotbar: [InventorySlot; 10],
    pub selected: usize,
    pub inner_rows: Vec<[InventorySlot; 10]>,
    pub equipment: Equipment,

}

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub helmet: Option<ItemStack>,
    pub chest: Option<ItemStack>,
    pub legs: Option<ItemStack>,
    pub boots: Option<ItemStack>,
    pub offhand: Option<ItemStack>,
}

#[derive(Clone, Debug, Default)]
pub struct InventorySlot {
    pub stack: Option<ItemStack>,
}

#[derive(Clone, Debug)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Clone, Default, Debug, Component)]
pub struct Item {
    pub stack_size: u32,
    pub item: ItemDef,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ItemId {
    Wood,
    Stone,
    IronOre,
    IronIngot,
    IronSword,
    HealthPotion,
    #[default]
    Thing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Resource,
    Armor,
    Weapon,
    Tool,
    Consumable,
    MazeUtil,
    Food,
    Waste,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    #[default]
    F,
}

#[derive(Clone, Debug, Default)]
pub struct ItemDef {
    pub id: ItemId,
    pub label: &'static str,
    pub asset: &'static str,
    pub category: Vec<Category>,
    pub tier: Tier,
    pub max_stack: u32,
}

impl Tier {
    //in prozent
    pub fn chance(self) -> f32 {
        match self {
            Tier::S => { 0.1 }
            Tier::A => { 0.9 }
            Tier::B => { 2.0 }
            Tier::C => { 5.0 }
            Tier::D => { 30.0 }
            Tier::F => { 60.0 }
        }
    }
}


//pub const ITEMS: &[ItemDef];




