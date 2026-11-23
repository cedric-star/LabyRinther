use crate::ingame::entity::structs::*;

impl Default for Health {
    fn default() -> Self {
        Self {
            health_points: 100.0,
            max_health: 100.0,
        }
    }
}

impl Default for Mana {
    fn default() -> Self {
        Self {
            mana_points: 0.0,
            max_mana: 100.0,
        }
    }
}




