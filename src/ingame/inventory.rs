use bevy::prelude::*;
use crate::ingame::player::*;
use crate::ingame::ui::*;
use crate::ingame::inventory_meta::*;
use crate::settings::settings_screen::*;
use crate::util::ui::constants::*;
use bevy::window::PrimaryWindow;
use crate::ingame::items::*;

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
pub struct InventoryRoot;

#[derive(Component)]
pub struct InventoryShow;

#[derive(Component)]
pub struct InventorySlotNormalCords(usize, usize);

#[derive(Component)]
pub struct InventorySlotItemNormalCords(usize, usize);

#[derive(Component)]
pub struct InventorySlotUsablesCords(usize, usize);

#[derive(Component)]
pub struct InventorySlotEquip(pub EquipSlot);

#[derive(Component)]
pub struct InventorySlotSecret(usize);

#[derive(Component)]
pub struct InventoryPlayerView;

#[derive(Component)]
pub struct InventorySlotCount(usize, usize);

#[derive(Component)]
pub struct CursorItemVisual;

#[derive(Component)]
pub struct CursorCountText;

#[derive(Resource, Default)]
pub struct CursorItem(pub Option<ItemStack>);

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
        InventoryRoot,
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
                                Interaction::default(),
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
                                    children![(
                                        InventorySlotCount(i_width, i_height),
                                        Text::new(""),
                                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                                        Node {
                                            position_type: PositionType::Absolute,
                                            right: px(2),
                                            bottom: px(0),
                                            ..default()
                                        }
                                    )]
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
                    Interaction::default(),
                    InventorySlotEquip(EquipSlot::Helm),
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(0.0),
                        top: percent(0.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                    children![(
                        InventoryEquipItem(EquipSlot::Helm),
                        Node {
                            position_type: PositionType::Absolute,
                            width: percent(100),
                            height: percent(100),
                            ..default() },
                        ImageNode::default(),
                        Visibility::Hidden,
                    )]
                )),
                Spawn((
                    InventorySlotEquip(EquipSlot::Chest),
                    Interaction::default(),
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(0.0),
                        top: percent(10.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                    children![(
                        InventoryEquipItem(EquipSlot::Chest),
                        Node {
                            position_type: PositionType::Absolute,
                            width: percent(100),
                            height: percent(100),
                            ..default() },
                        ImageNode::default(),
                        Visibility::Hidden,
                    )]
                )),
                Spawn((
                    InventorySlotEquip(EquipSlot::Legs),
                    Interaction::default(),
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(30.0),
                        top: percent(0.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                    children![(
                        InventoryEquipItem(EquipSlot::Legs),
                        Node {
                            position_type: PositionType::Absolute,
                            width: percent(100),
                            height: percent(100),
                            ..default() },
                        ImageNode::default(),
                        Visibility::Hidden,
                    )]
                )),
                Spawn((
                    InventorySlotEquip(EquipSlot::Boots),
                    Interaction::default(),
                    Node {
                        position_type: PositionType::Absolute,
                        width: percent(10.0),
                        height: percent(10.0),
                        left: percent(30.0),
                        top: percent(10.0),
                        ..default()
                    },
                    ImageNode::new(slot_pic.clone()),
                    children![(
                        InventoryEquipItem(EquipSlot::Boots),
                        Node {
                            position_type: PositionType::Absolute,
                            width: percent(100),
                            height: percent(100),
                            ..default() },
                        ImageNode::default(),
                        Visibility::Hidden,
                    )]
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

    commands.spawn((
        IngameUi,
        CursorItemVisual,
        Node {
            position_type: PositionType::Absolute,
            width: px(48),
            height: px(48),
            ..default()
        },
        ImageNode::default(),
        GlobalZIndex(100),
        Visibility::Hidden,
        children![(
            CursorCountText,
            Text::new(""),
            TextFont { font_size: FontSize::Px(14.0), ..default() },
            Node {
                position_type: PositionType::Absolute,
                right: px(2),
                bottom: px(0),
                ..default()
            },
        )],
    ));
}

pub fn update_inventory(
    player: Query<&Player>,
    mut slots: Query<
        (&InventorySlotNormalCords, &mut ImageNode, &mut Visibility),
        Without<InventorySlotItemNormalCords>,
    >,
    mut item_slots: Query<
        (&InventorySlotItemNormalCords, &mut ImageNode, &mut Visibility),
        Without<InventorySlotNormalCords>,
    >,
    mut counts: Query<(&InventorySlotCount, &mut Text)>,
    asset_server: Res<AssetServer>,
    items: Res<Items>,
) {
    let Ok(player) = player.single() else { return; };
    let inv = &player.inventory;

    // nur existierende Reihen anzeigen
    for (cords, mut image, mut vis) in &mut slots {
        let show = cords.1 == HOTBAR_ROW || cords.1 >= inv.row_offset();
        vis.set_if_neq(if show { Visibility::Inherited } else { Visibility::Hidden });
        //image.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
    }

    for (cords, mut image, mut vis) in &mut item_slots {
        match inv.slot(cords.0, cords.1).and_then(|s| s.stack) {
            Some(item) => {
                vis.set_if_neq(Visibility::Inherited);
                let handle = asset_server.load(items.0.get(&item.id).unwrap().asset.clone());
                if image.image != handle {
                    image.image = handle;
                }
            }
            None => {
                vis.set_if_neq(Visibility::Hidden);
            }
        }
    }

    for (cords, mut text) in &mut counts {
        let n = inv.slot(cords.0, cords.1).and_then(|s| s.stack).map_or(0, |s| s.count);
        text.0 = if n > 1 { n.to_string() } else { String::new() };
    }
}
pub fn update_inventory_items(
    player: Query<&Player>,
    mut hotbar: Query<(&HotBarItem, &mut ImageNode, &mut Visibility), Without<HotBarItemNum>>,
    mut hotbar_item: Query<(&HotBarItemNum, &mut Text, &mut Visibility), Without<HotBarItem>>,
    asset_server: Res<AssetServer>,
    items: Res<Items>,
) {
    let Ok(player) = player.single() else {
        return; // kein oder mehrere Spieler
    };

    for (slot, mut image, mut visibility) in &mut hotbar {
        let i = slot.0;
        match &player.inventory.hotbar[i].stack {
            Some(item) => {
                *visibility = Visibility::Visible;
                let handle = asset_server.load(items.0.get(&item.id).unwrap().asset.clone());
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

//inventar klicken:
pub fn inventory_click(
    mouse: Res<ButtonInput<MouseButton>>,
    grid_slots: Query<(&InventorySlotNormalCords, &Interaction)>,
    equip_slots: Query<(&InventorySlotEquip, &Interaction)>,
    root: Query<&Visibility, With<InventoryRoot>>,
    mut player: Query<&mut Player>,
    mut cursor: ResMut<CursorItem>,
    items: Res<Items>,
) {
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    if !left && !right { return; }

    let Ok(vis) = root.single() else { return; };
    if *vis == Visibility::Hidden { return; }
    let Ok(mut player) = player.single_mut() else { return; };
    let inv = &mut player.inventory;

    if let Some((eq, _)) = equip_slots.iter().find(|(_, i)| **i != Interaction::None) {
        let slot = inv.equip_mut(eq.0);
        apply_click(&mut cursor.0, slot, left, Some(eq.0), &items);
    } else if let Some((c, _)) = grid_slots.iter().find(|(_, i)| **i != Interaction::None) {
        let Some(slot) = inv.slot_mut(c.0, c.1) else { return; };
        apply_click(&mut cursor.0, &mut slot.stack, left, None, &items);
    }
}

fn apply_click(
    held: &mut Option<ItemStack>,
    slot: &mut Option<ItemStack>,
    left: bool,
    restriction: Option<EquipSlot>,
    items: &Items,
) {
    if left {
        match (*held, *slot) {
            (None, s) => { *held = s; *slot = None; }
            // Ablegen nur, wenn der Slot das Item akzeptiert
            (Some(h), _) if !can_place(restriction, h.id, &items) => {}
            (Some(h), None) => { *slot = Some(h); *held = None; }
            (Some(h), Some(s)) if h.id == s.id => {
                let max = items.0.get(&s.id).unwrap().max_stack;
                let total = h.count + s.count;
                let put = total.min(max);
                *slot = Some(ItemStack { id: s.id, count: put });
                *held = (total > put).then(|| ItemStack { id: h.id, count: total - put });
            }
            (Some(h), Some(s)) => { *slot = Some(h); *held = Some(s); } // tauschen
        }
    } else {
        match (*held, *slot) {
            (None, Some(s)) => {
                *held = Some(ItemStack { id: s.id, count: 1 });
                *slot = (s.count > 1).then(|| ItemStack { id: s.id, count: s.count - 1 });
            }
            (Some(h), Some(s)) if h.id == s.id => {
                if h.count < items.0.get(&h.id).unwrap().max_stack {
                    *held = Some(ItemStack { id: h.id, count: h.count + 1 });
                    *slot = (s.count > 1).then(|| ItemStack { id: s.id, count: s.count - 1 });
                }
            }
            _ => {}
        }
    }
}
pub fn update_cursor_visual(
    cursor: Res<CursorItem>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut visual: Query<(&mut Node, &mut ImageNode, &mut Visibility), With<CursorItemVisual>>,
    mut text: Query<&mut Text, With<CursorCountText>>,
    asset_server: Res<AssetServer>,
    items: Res<Items>,
) {
    let Ok((mut node, mut image, mut vis)) = visual.single_mut() else { return; };
    let pos = windows.single().ok().and_then(|w| w.cursor_position());

    match (cursor.0, pos) {
        (Some(item), Some(pos)) => {
            vis.set_if_neq(Visibility::Visible);
            node.left = px(pos.x - 24.0);
            node.top = px(pos.y - 24.0);
            let handle = asset_server.load(items.0.get(&item.id).unwrap().asset.clone());
            if image.image != handle {
                image.image = handle;
            }
            if let Ok(mut t) = text.single_mut() {
                t.0 = if item.count > 1 { item.count.to_string() } else { String::new() };
            }
        }
        _ => {
            vis.set_if_neq(Visibility::Hidden);
        }
    }
}

#[derive(Component)]
pub struct InventoryEquipItem(pub EquipSlot);

pub fn update_equipment(
    player: Query<&Player>,
    mut items: Query<(&InventoryEquipItem, &mut ImageNode, &mut Visibility)>,
    asset_server: Res<AssetServer>,
    items_res: Res<Items>,
) {
    let Ok(player) = player.single() else { return; };
    let e = &player.inventory.equipment;
    for (slot, mut image, mut vis) in &mut items {
        let stack = match slot.0 {
            EquipSlot::Helm => e.helm,
            EquipSlot::Chest => e.chest,
            EquipSlot::Legs => e.legs,
            EquipSlot::Boots => e.boots,
            EquipSlot::Usable => e.usable,
            EquipSlot::Secret => e.secret,
        };
        match stack {
            Some(it) => {
                vis.set_if_neq(Visibility::Inherited);
                let h = asset_server.load(items_res.0.get(&it.id).unwrap().asset.clone());
                if image.image != h { image.image = h; }
            }
            None => { vis.set_if_neq(Visibility::Hidden); }
        }
    }
}
