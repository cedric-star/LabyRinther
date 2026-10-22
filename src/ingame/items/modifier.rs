use serde::{Serialize, Deserialize};

// hier liegen dinge für items, die stats eines player verändern

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Stat {
    MaxHealth,
    MaxMana,
    InventoryRows,
    MoveSpeed,
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