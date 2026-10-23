use bevy::prelude::*;
use crate::ingame::player::structs::*;
use bevy::window::PrimaryWindow;
use crate::ingame::items::structs::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::util::*;
use crate::ingame::structs::*;
use crate::ingame::items::modifier::*;

//setzt kästchen in hotbar um ausgeählten slot und setzt es im inventory struct
// aktiviert auc mit e inventar
pub fn set_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    mut inventory: Single<&mut Inventory, With<Player>>,
    mut inventory_show: Query<(&mut InventoryShow, &mut Visibility)>,
) {

    if keys.pressed(KeyCode::Digit1) {
        inventory.selected = 0;
    } else if keys.pressed(KeyCode::Digit2) {
        inventory.selected = 1;
    } else if keys.pressed(KeyCode::Digit3) {
        inventory.selected = 2;
    } else if keys.pressed(KeyCode::Digit4) {
        inventory.selected = 3;
    } else if keys.pressed(KeyCode::Digit5) {
        inventory.selected = 4;
    } else if keys.pressed(KeyCode::Digit6) {
        inventory.selected = 5;
    } else if keys.pressed(KeyCode::Digit7) {
        inventory.selected = 6;
    } else if keys.pressed(KeyCode::Digit8) {
        inventory.selected = 7;
    } else if keys.pressed(KeyCode::Digit9) {
        inventory.selected = 8;
    } else if keys.pressed(KeyCode::Digit0) {
        inventory.selected = 9;
    }

    // go to inventory
    if keys.just_pressed(KeyCode::KeyE) {
        for (mut inv, mut visibility) in &mut inventory_show {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
                inv.0 = true;
            } else {
                *visibility = Visibility::Hidden;
                inv.0 = false;
            }
        }
    }

}

//aktualisiert anzeige des inventars (aktualisiert items im inventar)
//aktualisiert auch anzahl an inventar/slot reihen
pub fn update_inventory(
    inventory: Single<&Inventory, With<Player>>,
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
    // nur existierende Reihen anzeigen
    for (cords, mut image, mut vis) in &mut slots {
        let show = cords.1 == HOTBAR_ROW || cords.1 >= inventory.row_offset();
        vis.set_if_neq(if show { Visibility::Inherited } else { Visibility::Hidden });
        //image.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
    }

    for (cords, mut image, mut vis) in &mut item_slots {
        match inventory.slot(cords.0, cords.1).and_then(|s| s.stack) {
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
        let n = inventory.slot(cords.0, cords.1).and_then(|s| s.stack).map_or(0, |s| s.count);
        text.0 = if n > 1 { n.to_string() } else { String::new() };
    }
}

//setzt items in hotbar + stückzahl setzen
pub fn update_inventory_items(
    inventory: Single<&Inventory, With<Player>>,
    mut hotbar: Query<(&HotBarItem, &mut ImageNode, &mut Visibility), Without<HotBarItemNum>>,
    mut hotbar_item: Query<(&HotBarItemNum, &mut Text, &mut Visibility), Without<HotBarItem>>,
    asset_server: Res<AssetServer>,
    items: Res<Items>,
) {
    for (slot, mut image, mut visibility) in &mut hotbar {
        let i = slot.0;
        match inventory.hotbar[i].stack {
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
        match inventory.hotbar[slot].stack {
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

// zeigt passenden stack aus equipment an (rüstung,  usables, secrets)
pub fn update_equipment_ui(
    e: Single<&Equipment, With<Player>>,
    mut items: Query<(&InventoryEquipItem, &mut ImageNode, &mut Visibility)>,
    asset_server: Res<AssetServer>,
    items_res: Res<Items>,
) {
    for (slot, mut image, mut vis) in &mut items {
        let stack = match slot.0 {
            EquipSlot::Helm => e.helm,
            EquipSlot::Chest => e.chest,
            EquipSlot::Legs => e.legs,
            EquipSlot::Boots => e.boots,
            EquipSlot::Usable(i) => e.usable[i],
            EquipSlot::Secret(i) => e.secret[i],
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

//inventar klicken:
//erkennt einen klick im inventar/craftui
// -> passende items aus slot geholt und in schwebenden modus gespeichert / oder wieder rausgeholt
pub fn inventory_click(
    mouse: Res<ButtonInput<MouseButton>>,
    grid_slots: Query<(&InventorySlotNormalCords, &Interaction)>,
    equip_slots: Query<(&InventorySlotEquip, &Interaction)>,
    root: Query<&Visibility, With<InventoryRoot>>,
    player: Single<(&mut Inventory, &mut Equipment), With<Player>>,
    mut cursor: ResMut<CursorItem>,
    items: Res<Items>,
) {
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    if !left && !right { return; }

    let Ok(vis) = root.single() else { return; };
    if *vis == Visibility::Hidden { return; }

    let (mut inventory, mut equipment) = player.into_inner();

    if let Some((eq, _)) = equip_slots.iter().find(|(_, i)| **i != Interaction::None) {
        let slot = equipment.equip_mut(eq.0);
        apply_click(&mut cursor.0, slot, left, Some(eq.0), &items);
    } else if let Some((c, _)) = grid_slots.iter().find(|(_, i)| **i != Interaction::None) {
        let Some(slot) = inventory.slot_mut(c.0, c.1) else { return; };
        apply_click(&mut cursor.0, &mut slot.stack, left, None, &items);
    }
}


//aktualisiert schwebende anzeige im ui
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

pub fn update_euqipped_info_text(
    windows: Query<&Window, With<PrimaryWindow>>,
    equips: Single<&Equipment, With<Player>>,
    items: Res<Items>,
    equip_slots: Query<(&InventorySlotEquip, &Interaction)>,
    mut tooltip: Query<(&mut Node, &mut Text, &mut Visibility), With<CursorItemInfoText>>,   
) {
    let Ok((mut node, mut text, mut vis)) = tooltip.single_mut() else { return; };
    let Some(pos) = windows.single().ok().and_then(|w| w.cursor_position()) else {return;};
    
    let Some((hovered_slot, _)) = equip_slots //hovered slot finden
        .iter()
        .find(|(_, interaction)| **interaction == Interaction::Hovered)
    else {
        return;
    };

    let Some(equip_stack) = (match hovered_slot {//player inventar in slots finden
        InventorySlotEquip(EquipSlot::Helm) => equips.helm,
        InventorySlotEquip(EquipSlot::Chest) => equips.chest,
        InventorySlotEquip(EquipSlot::Legs) => equips.legs,
        InventorySlotEquip(EquipSlot::Boots) => equips.boots,
        InventorySlotEquip(EquipSlot::Usable(i)) => equips.usable[*i],
        InventorySlotEquip(EquipSlot::Secret(i)) => equips.secret[*i],
    }) else {
        return;
    };
    
    let Some(def) = items.0.get(&equip_stack.id) else { return; };
    let label = format!("{}\n{:?}", def.label, def.categories);
    if text.0 != label { text.0 = label; }
    node.left = px(pos.x + 16.0);
    node.top = px(pos.y + 16.0);
    vis.set_if_neq(Visibility::Visible);
        
}

//zeigt tooltip über hover an, bei normalen inventaritems
pub fn update_inventory_info_text(
    windows: Query<&Window, With<PrimaryWindow>>,
    inventory: Single<&Inventory, With<Player>>,
    items: Res<Items>,
    inv_slots: Query<(&InventorySlotNormalCords, &Interaction)>,
    mut tooltip: Query<(&mut Node, &mut Text, &mut Visibility), With<CursorItemInfoText>>,
) {
    let Ok((mut node, mut text, mut vis)) = tooltip.single_mut() else { return; };

    let pos = windows.single().ok().and_then(|w| w.cursor_position());
    let hovered_stack = inv_slots
        .iter()
        .find(|(_, i)| **i != Interaction::None)
        .and_then(|(c, _)| inventory.slot(c.0, c.1))
        .and_then(|slot| slot.stack);

    let (Some(stack), Some(pos)) = (hovered_stack, pos) else {
        vis.set_if_neq(Visibility::Hidden);
        return;
    };

    let Some(def) = items.0.get(&stack.id) else { return; };
    let label = format!("{}\n{:?}\n({}/{})", def.label, def.categories, stack.count, def.max_stack);
    if text.0 != label { text.0 = label; }
    node.left = px(pos.x + 16.0);
    node.top = px(pos.y + 16.0);
    vis.set_if_neq(Visibility::Visible);
}
