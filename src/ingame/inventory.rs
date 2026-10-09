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

#[derive(Component)]
pub struct InventorySlotNormalCords(usize, usize);

#[derive(Component)]
pub struct InventorySlotItemNormalCords(usize, usize);

#[derive(Component)]
pub struct InventorySlotUsablesCords(usize, usize);

#[derive(Component)]
pub struct InventorySlotHelm;

#[derive(Component)]
pub struct InventorySlotChest;

#[derive(Component)]
pub struct InventorySlotLegs;

#[derive(Component)]
pub struct InventorySlotBoots;

#[derive(Component)]
pub struct InventorySlotSecret(usize);

#[derive(Component)]
pub struct InventoryPlayerView;

pub fn spawn_inventory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    s: Res<GameSettings>,
) {
    let scale = s.ui_scale * 0.7;
    let slot_pic: Handle<Image> = asset_server.load("ingame_ui/item_slot.png");
    let pic_normal = slot_pic.clone();
    let pic_usables = slot_pic.clone();

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
                height: percent(35.0 * scale),
                max_height: percent(105.0),
                aspect_ratio: Some(1.0),
                justify_self: JustifySelf::Center,
                ..default()
            },
            BackgroundColor(BG_COL_A),
            Children::spawn((

                // normales Inventar + Hotbar
                SpawnIter((0..=9).map(move |i_width| {
                    let slot_pic = pic_normal.clone();
                    (
                        Node {
                            position_type: PositionType::Absolute,
                            width: percent(10.0),
                            height: percent(100.0),
                            left: percent(i_width as f32 * 10.0),
                            ..default()
                        },
                        Children::spawn(SpawnIter((0..=6).map(move |i_height| {
                            let top = if i_height >= 6 {
                                25.0 + 5.0 + 10.0 * i_height as f32
                            } else {
                                25.0 + 10.0 * i_height as f32
                            };
                            (
                                Node {
                                    position_type: PositionType::Absolute,
                                    width: percent(100.0),
                                    height: percent(10.0),
                                    top: percent(top),
                                    ..default()
                                },
                                InventorySlotNormalCords(i_width, i_height),
                                ImageNode::new(slot_pic.clone()),
                                children![(
                                    InventorySlotItemNormalCords(i_width, i_height),
                                    Node {
                                        position_type: PositionType::Absolute,
                                        width: percent(100),
                                        height: percent(100.0),
                                        ..default()
                                    },
                                    ImageNode::default(),
                                    Visibility::Hidden,
                                )]
                            )
                        }))),
                    )
                })),

                // Usables
                SpawnIter((0..=3).map(move |i_width| {
                    let slot_pic = pic_usables.clone();
                    (
                        Node {
                            position_type: PositionType::Absolute,
                            width: percent(10.0),
                            height: percent(100.0),
                            left: percent(60.0 + 10.0 * i_width as f32),
                            top: percent(0.0),
                            ..default()
                        },
                        Children::spawn(SpawnIter((0..=1).map(move |i_height| (
                            InventorySlotUsablesCords(i_width, i_height),
                            Node {
                                position_type: PositionType::Absolute,
                                width: percent(100.0),
                                height: percent(10.0),
                                top: percent(10.0 * i_height as f32),
                                ..default()
                            },
                            ImageNode::new(slot_pic.clone()),
                        )))),
                    )
                })),

                //sercret slots uiuiui kiki ein reh auf der straße uiuiui
                Spawn((
                    InventorySlotSecret(0),
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(45.0),
                        top: percent(0.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                )),
                Spawn((
                    InventorySlotSecret(1),
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(45.0),
                        top: percent(10.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                )),

                //helm and chest and legs and boots
                Spawn((
                    InventorySlotHelm,
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(0.0),
                        top: percent(0.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                )),
                Spawn((
                    InventorySlotChest,
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(0.0),
                        top: percent(10.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                )),
                Spawn((
                    InventorySlotLegs,
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(30.0),
                        top: percent(0.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                )),
                Spawn((
                    InventorySlotBoots,
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(30.0),
                        top: percent(10.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                )),

                //player preview
                Spawn((
                    InventoryPlayerView,
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(20.0),
                        height: percent(20.0),
                        left: percent(10.0),
                        top: percent(0.0),
                        ..default()
                    },
                    BackgroundColor(BG_COL),
                )),

            )),
        )],
    ));
}
pub fn update_inventory(
    player: Query<&Player>,
    mut slots: Query<(&InventorySlotNormalCords, &mut ImageNode), Without<InventorySlotItemNormalCords>>,
    mut item_slots: Query<(&InventorySlotItemNormalCords, &mut ImageNode, &mut Visibility), Without<InventorySlotNormalCords>>,
) {
    let Ok(player) = player.single() else { return; };
    let inventory_size = player.inventory.inner_rows.len();
    for (cords, mut image) in &mut slots {
        if cords.1 <=  (inventory_size + 1) { //+1 wegen hotbar
            image.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
        }
    }

    for (cords, mut image, mut visiility) in &mut item_slots {
        //hier item icons visible machen und eintragen, dafür player inventar durchsuchen
        //hier fehlt code
    }
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
