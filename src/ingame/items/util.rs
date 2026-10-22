use crate::ingame::items::structs::*;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;

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