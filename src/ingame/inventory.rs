use bevy::prelude::*;
use crate::ingame::player::*;
use crate::ingame::ui::*;
use crate::ingame::inventory_meta::*;

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

pub fn update_inventory_items(
    player: Query<&Player>,
    mut hotbar: Query<(&HotBarItem, &mut ImageNode, &mut Visibility)>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player) = player.single() else {
        return; // kein oder mehrere Spieler
    };

    for (slot, mut image, mut visibility) in &mut hotbar {
        let i = slot.0;
        match &player.inventory.hotbar[i].stack {
            Some(item) => {
                *visibility = Visibility::Visible;
                let handle = asset_server.load(get_item_by_id(item.id).asset);
                if image.image != handle {
                    image.image = handle;
                }
            },
            None => {
                *visibility = Visibility::Hidden;
            },
        }
    }
}
