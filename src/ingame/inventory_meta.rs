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
    Apple,
    JucyWoddenDagger,
    WoddenDagger,
    #[default]
    Thing,
    Nothing,
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
    Nothing,
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
    pub categories: &'static [Category],
    pub tier: Tier,
    pub max_stack: u32,
    //optionale attribute:
    pub weapon: Option<WeaponDef>,
}

#[derive(Clone, Copy, Debug)]
pub struct WeaponDef {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub wtype: WeaponType,
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponType {
    Sword,
    Staff,
    Shooting,
    Whip,
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

pub struct Recipe {
    pub items: &'static [(ItemId, i32)], //item + anzahl des items
    pub station: Option<CraftStation>,
}

pub struct CraftStation;

pub const RECIOES: &[Recipe; 1] = &[
    Recipe {
        items: &[
            (ItemId::WoddenDagger, 1),
            (ItemId::Apple, 2),
        ],
        station: None,
    }
];

pub const ITEMS: &[ItemDef; 4] = &[
        ItemDef {
        id: ItemId::Nothing,
        label: "Error",
        asset: "items/Error.png",
        categories: &[Category::Nothing],
        tier: Tier::F,
        max_stack: 1,
        weapon: None,
    },
    ItemDef {
        id: ItemId::Apple,
        label: "Apple",
        asset: "items/Apple.png",
        categories: &[Category::Food, Category::Resource],
        tier: Tier::F,
        max_stack: 999,
        weapon: None
    },
    ItemDef {
        id: ItemId::WoddenDagger,
        label: "Wodden Dagger",
        asset: "items/WoddenDagger.png",
        categories: &[Category::Resource, Category::Weapon],
        tier: Tier::F,
        max_stack: 1,
        weapon: Some(WeaponDef {
            damage: 10.0,
            range: 2.0,
            attack_speed: 1.0,
            wtype: WeaponType::Sword,
        }),
    },
    ItemDef {
        id: ItemId::JucyWoddenDagger,
        label: "Jucy Wodden Dagger",
        asset: "items/JucyWoddenDagger.png",
        categories: &[Category::Weapon],
        tier: Tier::D,
        max_stack: 1,
        weapon: Some(WeaponDef {
            damage: 12.0,
            range: 2.0,
            attack_speed: 1.5,
            wtype: WeaponType::Sword,
        }),
    },
];

pub fn get_item_by_id(id: ItemId) -> ItemDef {
    for item in ITEMS {
        if item.id == id {
            return item.clone();
        }
    }
    return ITEMS[0].clone();
}
