use bevy::prelude::*;
use crate::ingame::player::*;
use crate::ingame::ui::*;
use crate::ingame::inventory_meta::*;
use crate::settings::settings_screen::*;
use crate::util::ui::constants::*;

pub fn set_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Player>,
    mut inventory_show: Query<(&InventoryShow, &mut Visibility)>,
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

    // go to inventory
    if keys.just_pressed(KeyCode::KeyE) {
        for (inv, mut visibility) in &mut inventory_show {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }

}

#[derive(Component)]
pub struct InventoryShow;

pub fn spawn_inventory(
    mut commands: Commands,
    asset_Server: Res<AssetServer>,
    s: Res<GameSettings>,
) {
    let scale = s.ui_scale;
    commands.spawn((
        IngameUi,
        InventoryShow,
        Node {
            display: Display::Grid,
            width: percent(100.0),
            height: percent(100.0),
            grid_template_columns: vec![
                GridTrack::percent(20.0),
                GridTrack::flex(1.0),
                GridTrack::percent(20.0),
            ],
            grid_template_rows: vec![
                GridTrack::percent(20.0),
                GridTrack::flex(1.0),
                GridTrack::percent(20.0),
            ],
            ..default()
        },
        Visibility::Hidden,
        children![(
            Node {
                grid_column: GridPlacement::start(2),
                grid_row: GridPlacement::start(2),
                height: percent(35.0 * (scale/2.0)),
                max_height: percent(105.0),
                aspect_ratio: Some(1.2),
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor(BG_COL_A),
        )],
    ));
}

pub fn update_inventory(

) {

}

pub fn update_inventory_items(
    player: Query<&Player>,
    mut hotbar: Query<(&HotBarItem, &mut ImageNode, &mut Visibility), Without<HotBarItemNum>>,
    mut hotbar_item: Query<(&HotBarItemNum, &mut Text, &mut Visibility), Without<HotBarItem>>,
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

    for (num_item, mut text, mut visibility) in &mut hotbar_item {
        let slot = num_item.0;
        match &player.inventory.hotbar[slot].stack {
            Some(item) => {
                *visibility = Visibility::Visible;
                text.clear();
                text.push_str(item.count.to_string().as_str());
            }
            None => {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
