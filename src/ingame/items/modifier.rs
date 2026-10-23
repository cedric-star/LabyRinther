use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use std::collections::HashMap; 
use crate::ingame::items::structs::*;
use crate::ingame::inventory::structs::*;
// hier liegen dinge für items, die stats eines player verändern

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stat {
    MaxHealth,
    MaxMana,
    HealthRegen,
    ManaRegen,
    InventoryRows,
    MoveSpeed,
    Armor,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Modifier {
    Add(f32),
    Mul(f32),
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EffectTrigger {
    Equipped, //in usable slot im inventar
    OnUse, //bei rechtklick wenn item in der hand
    
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatEffect {
    pub stat: Stat,
    pub modifier: Modifier,
    pub trigger: EffectTrigger,
}

#[derive(Component)]
pub struct Stats {
    pub base: HashMap<Stat, f32>,
    pub permanent: Vec<(Stat, Modifier)>, //permanent
    pub equipped: Vec<(Stat, Modifier)>, //je nach equipped sachen
}

impl Default for Stats {
    fn default() -> Self {
        let base = HashMap::from([
            (Stat::MaxHealth, 100.0),
            (Stat::MaxMana, 100.0),
            (Stat::HealthRegen, 1.0),
            (Stat::ManaRegen, 1.0),
            (Stat::InventoryRows, 2.0),
            (Stat::MoveSpeed, 1.0),
        ]);
        Self {base, permanent: vec![], equipped: vec![] }
    }
}

impl Stats {
    pub fn get(&self, stat: Stat) -> f32 {
        let base = self.base.get(&stat).copied().unwrap_or(0.0);
        let (mut add, mut mul) = (0.0, 0.0);
        for (s, m) in self.permanent.iter().chain(&self.equipped) {
            if *s != stat { continue; }
            match m {
                Modifier::Add(v) => add += v,
                Modifier::Mul(v) => mul += v,
            }
        }
        (base + add) * (1.0 + mul)
    }
}


#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub helm: Option<ItemStack>,
    pub chest: Option<ItemStack>,
    pub legs: Option<ItemStack>,
    pub boots: Option<ItemStack>,
    pub usable: [Option<ItemStack>; 8],
    pub secret: [Option<ItemStack>; 2],
}

impl Equipment {
    pub fn iter_stacks(&self) -> impl Iterator<Item = &ItemStack> {
        [&self.helm, &self.chest, &self.legs, &self.boots]
            .into_iter()
            .chain(self.usable.iter())
            .chain(self.secret.iter())
            .filter_map(|s| s.as_ref())
    }

    pub fn equip_mut(&mut self, s: EquipSlot) -> &mut Option<ItemStack> {
        match s {
            EquipSlot::Helm     => &mut self.helm,
            EquipSlot::Chest    => &mut self.chest,
            EquipSlot::Legs     => &mut self.legs,
            EquipSlot::Boots    => &mut self.boots,
            EquipSlot::Usable(i)   => &mut self.usable[i],
            EquipSlot::Secret(i)   => &mut self.secret[i],
        }
    }
}