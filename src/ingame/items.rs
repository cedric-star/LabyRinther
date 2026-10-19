use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;


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

    pub fn col(self, alpha: f32) -> Color {
        match self {
            Tier::S => Color::srgba(100.0, 0.0, 42.0, alpha), //rot
            Tier::A => Color::srgba(100.0, 50.0, 0.0, alpha), //orange
            Tier::B => Color::srgba(61.0, 0.0, 100.0, alpha), //lila
            Tier::C => Color::srgba(0.0, 5.0, 100.0, alpha),  //blau
            Tier::D => Color::srgba(0.0, 100.0, 20.0, alpha), //grün
            Tier::F => Color::srgba(75.0, 75.0, 75.0, alpha), //grau
        }
    }
}

#[derive(Resource)]
pub struct Items(pub HashMap<ItemId, ItemDef>);

impl Default for Items {
    fn default() -> Self {
        let path = "./items.json";
        let file = match fs::read_to_string(path) {
            Ok(file) => {
                if file.trim().is_empty() { panic!("items json file is leer ):"); }
                file
            },
            Err(e) => {
                panic!("items json file nicht geladen ):");
            },
        };

        match serde_json::from_str::<Vec<ItemDef>>(&file) {
            Ok(list) => Self(list.into_iter().map(|item| (item.id, item)).collect()),
            Err(e) => { panic!("json items ungültig ):"); }
        }

    }
}
