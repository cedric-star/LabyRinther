use crate::ingame::items::attack::structs::*;

impl Default for Attacker {
    fn default() -> Self {
        Self {
            active: None,
            cooldown_left: 0.0,               
        }
    }
}