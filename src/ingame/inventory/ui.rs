use bevy::prelude::*;
use crate::ingame::player::ui::*;
use crate::ingame::player::util::*;
use crate::ingame::player::systems::*;
use crate::ingame::player::structs::*;
use crate::ingame::ui::*;
use crate::ingame::structs::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::systems::*;
use crate::settings::settings_screen::*;
use crate::util::ui::constants::*;
use bevy::window::PrimaryWindow;
use crate::ingame::items::*;
use crate::constants::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::fs;



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