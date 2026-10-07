use bevy::prelude::*;
use crate::ingame::player::*;

pub fn set_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Player>,
) {
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    if keys.pressed(KeyCode::Digit1) {
        player.inventory.selected = 0;
    } else if keys.pressed(KeyCode::Digit2) {
        player.inventory.selected = 1;
    } else if keys.pressed(KeyCode::Digit3) {
        player.inventory.selected = 2;
    } else if keys.pressed(KeyCode::Digit4) {
        player.inventory.selected = 3;
    } else if keys.pressed(KeyCode::Digit5) {
        player.inventory.selected = 4;
    } else if keys.pressed(KeyCode::Digit6) {
        player.inventory.selected = 5;
    } else if keys.pressed(KeyCode::Digit7) {
        player.inventory.selected = 6;
    } else if keys.pressed(KeyCode::Digit8) {
        player.inventory.selected = 7;
    } else if keys.pressed(KeyCode::Digit9) {
        player.inventory.selected = 8;
    } else if keys.pressed(KeyCode::Digit0) {
        player.inventory.selected = 9;
    }

}
