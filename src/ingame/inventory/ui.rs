use bevy::prelude::*;
use crate::ingame::structs::*;
use crate::ingame::inventory::structs::*;
use crate::settings::settings_screen::*;
use crate::util::ui::constants::*;


//gibt bundle zurück, das inventar darstellt
pub fn get_inventory(
    asset_server: &AssetServer,
    s: &GameSettings,
) -> impl Bundle {
    let scale = s.ui_scale * 0.27;
    let slot_pic: Handle<Image> = asset_server.load("ingame_ui/item_slot.png");
    let pic_normal = slot_pic.clone();
    let pic_usables = slot_pic.clone();

    (
        IngameUi,
        InventoryShow(false),
        InventoryRoot,
        Node {
            top: percent( - (scale * 40.0)),
            position_type: PositionType::Absolute,
            grid_column: GridPlacement::start(2),
            grid_row: GridPlacement::start(2),
            display: Display::Grid,
            width: percent(140.0 * scale),
            height: percent(140.0 * scale),
            justify_self: JustifySelf::Center,
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

                // Usables: 8 Slots, Spalten ab x=60 %, 2 Reihen
                SpawnIter({
                    let pic = slot_pic.clone();
                    (0..=7).map(move |i| {
                        let (col, row) = (i / 2, i % 2);
                        equip_slot(pic.clone(), EquipSlot::Usable(i), 60.0 + 10.0 * col as f32, 10.0 * row as f32)
                    })
                }),

                // Secret-Slots
                Spawn(equip_slot(slot_pic.clone(), EquipSlot::Secret(0), 45.0, 0.0)),
                Spawn(equip_slot(slot_pic.clone(), EquipSlot::Secret(1), 45.0, 10.0)),

                // Rüstung
                Spawn(equip_slot(slot_pic.clone(), EquipSlot::Helm,  0.0,  0.0)),
                Spawn(equip_slot(slot_pic.clone(), EquipSlot::Chest, 0.0, 10.0)),
                Spawn(equip_slot(slot_pic.clone(), EquipSlot::Legs, 30.0,  0.0)),
                Spawn(equip_slot(slot_pic.clone(), EquipSlot::Boots,30.0, 10.0)),                

                
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
    )

}

/// Ein klickbarer Equip-Slot: Hintergrund-Node mit Interaction + Kind-Node für das Item-Bild.
/// `left`/`top` sind Prozent relativ zum Inventar-Container (der 10x10 Einheiten groß ist).
fn equip_slot(pic: Handle<Image>, slot: EquipSlot, left: f32, top: f32) -> impl Bundle {
    (
        InventorySlotEquip(slot),   // damit inventory_click den Slot findet
        Interaction::default(),     // damit Bevy Hover/Klick meldet
        Node {
            position_type: PositionType::Absolute,
            width: percent(10.0),
            height: percent(10.0),
            left: percent(left),
            top: percent(top),
            ..default()
        },
        ImageNode::new(pic),
        children![(
            InventoryEquipItem(slot), // damit update_equipment das Bild setzen kann
            Node {
                position_type: PositionType::Absolute,
                width: percent(100.0),
                height: percent(100.0),
                ..default()
            },
            ImageNode::default(),
            Visibility::Hidden,       // leer, bis ein Item drin liegt
        )],
    )
}



pub fn get_cursor(
    asset_server: &AssetServer,
    s: &GameSettings,
) -> impl Bundle {
    (
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
        ),],
    )
}

