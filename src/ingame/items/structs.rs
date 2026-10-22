use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Clone, Debug, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Clone, Default, Debug, Component)]
pub struct Item {
    pub stack_size: u32,
    pub item: ItemDef,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemId {
    IronChest,
    Apple,
    AppleSlice,
    JucyWoddenDagger,
    WoddenDagger,
    #[default]
    Thing,
    MediumDamagePotion,
    MediumHealthPotion,
    MediumMaxHealthOrb,
    IncrementInventoryRowsSecret,
    Nothing,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ItemDef {
    pub id: ItemId,
    pub label: String,
    pub asset: String,
    pub categories: Vec<Category>,
    pub tier: Tier,
    pub max_stack: u32,
    //optionale attribute:
    #[serde(default)]
    pub weapon: Option<WeaponDef>,
    #[serde(default)]
    pub armor: Option<ArmorDef>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct WeaponDef {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub wtype: WeaponType,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ArmorDef {
    pub points: f32, //max 15 bitti
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Staff,
    Shooting,
    Whip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Category {
    Resource,
    Armor,
    Helm,
    Chest,
    Legs,
    Boots,
    Weapon,
    Tool,
    Consumable,
    Usable,
    MazeUtil,
    Food,
    Waste,
    Secret,
    Nothing,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    #[default]
    F,
}



#[derive(Resource)]
pub struct Items(pub HashMap<ItemId, ItemDef>);


