use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;
use crate::ingame::items::structs::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::structs::*;
use crate::util::ui::constants::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::recipes::structs::*;
use crate::ingame::recipes::util::*;

/// Berechnet aus dem Inventar des Spielers, welche Rezepte aktuell craftbar sind,
/// und speichert sie samt maximal craftbarer Menge in `CraftableItems`.
pub fn set_craftable_items(
    inventory: Single<&Inventory, With<Player>>,
    mut craftable_items: ResMut<CraftableItems>,
    recipes: Res<Recipes>,
) {

    let mut player_items: Vec<ItemStack> = Vec::new();
    for inv_rows in inventory.inner_rows.clone() {
        for slot in inv_rows {
            if let Some(slot_unwrapped) = slot.stack {
                player_items.push(slot_unwrapped);
            }
        }
    }
    for slot in inventory.hotbar {
        if let Some(slot_unwrapped) = slot.stack {
            player_items.push(slot_unwrapped);
        }
    }

    let player_items = merge_stack(player_items);

    //alle craftbaren recipes:
    let mut craftable_recipes: Vec<ItemStack> = Vec::new();

    for recipe in recipes.0.clone() {
        match recipe.is_craftable(player_items.clone()) {
            Some(count) => { craftable_recipes.push(ItemStack { id: recipe.result.id, count: count}); }
            None => { continue; }
        }
    }

    //println!("recipe craftable: \n{:?}", craftable_recipes);

    craftable_items.0.clear();
    craftable_items.0 = craftable_recipes;

    return;
}

/// Hält die Craft-Slots in der UI synchron zur `CraftableItems`-Liste:
/// aktualisiert Bild, Hintergrundfarbe und Zahl, spawnt fehlende und entfernt überzählige Slots.
pub fn sync_craftable_ui(
    mut commands: Commands,
    craftable: Res<CraftableItems>,
    items: Res<Items>,
    asset_server: Res<AssetServer>,
    s: Res<GameSettings>,
    list: Query<Entity, With<CraftableList>>,
    mut slots: Query<(Entity, &CraftableSlot, &mut ImageNode, &mut BackgroundColor, &Children)>,
    mut counts: Query<(&mut Text, &mut Visibility), With<CraftableCountText>>,
    mut last: Local<Vec<ItemStack>>,
) {
    let Ok(list_entity) = list.single() else { return; };

    // vorhandene slots: bild + zahl aktualisieren
    let mut existing: HashMap<usize, Entity> = HashMap::new();
    for (entity, slot, mut image, mut bg, children) in &mut slots {
        existing.insert(slot.0, entity);
        let Some(stack) = craftable.0.get(slot.0) else { continue; };

        let handle = asset_server.load(items.0.get(&stack.id).unwrap().asset.clone());
        if image.image != handle { image.image = handle; }

        let new_bg = items.0.get(&stack.id).unwrap().tier.col(0.5);
        if bg.0 != new_bg { *bg = BackgroundColor(new_bg) }

        for child in children.iter() {
            if let Ok((mut text, mut vis)) = counts.get_mut(child) {
                *vis = if stack.count > 1 { Visibility::Inherited } else { Visibility::Hidden };
                let new = if stack.count > 1 { stack.count.to_string() } else { String::new() };
                if text.0 != new { text.0 = new; }
            }
        }
    }

    // fehlende slots spawnen
    for (i, stack) in craftable.0.iter().enumerate() {
        if existing.contains_key(&i) { continue; }
        let scale = s.ui_scale;
        let asset = items.0.get(&stack.id).unwrap().asset.clone();

        commands.entity(list_entity).with_children(|parent| {
            parent.spawn((
                CraftableSlot(i),
                Interaction::default(),
                Node {
                    width: percent(12.5),
                    aspect_ratio: Some(1.0),
                    ..default()
                },
                ImageNode::new(asset_server.load(asset)),
                children![
                    (
                        CraftableCountText,
                        Text::new(if stack.count > 1 { stack.count.to_string() } else { String::new() }),
                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                        TextColor(TXT_COL),
                        Node {
                            position_type: PositionType::Absolute,
                            right: px(2.0),
                            bottom: px(0.0),
                            ..default()
                        },
                        Visibility::Hidden,
                    ),
                    (
                        CraftableTooltip,
                        Node {
                            position_type: PositionType::Absolute,
                            left: px(50.0 * scale),
                            top: px(0.0),
                            padding: UiRect::all(px(4.0)),
                            min_width: px(120.0),
                            ..default()
                        },
                        BackgroundColor(BG_COL),
                        Text::new(""),
                        TextFont { font_size: FontSize::Px(12.0), ..default() },
                        TextColor(TXT_COL),
                        Visibility::Hidden,
                    )
                ],
            ));
        });
    }

    // überschüssige slots entfernen
    for (i, entity) in existing {
        if i >= craftable.0.len() {
            commands.entity(entity).despawn();
        }
    }
}

/// Zeigt beim Hovern über einen Craft-Slot den globalen Tooltip mit Zutaten und Ergebnis an.
pub fn craftable_hover(
    craftable: Res<CraftableItems>,
    recipes: Res<Recipes>,
    items: Res<Items>,
    slots: Query<(&CraftableSlot, &Interaction)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut tooltip: Query<(&mut Node, &mut Text, &mut Visibility), With<CursorCraftInfoText>>,
) {
    //tooltip daten
    let Ok((mut node, mut text, mut vis)) = tooltip.single_mut() else { return; };

    // genau den gehoverten slot suchen
    let hovered = slots.iter().find(|(_, i)| **i != Interaction::None);
    let pos = windows.single().ok().and_then(|w| w.cursor_position());

    let (Some((slot, _)), Some(pos)) = (hovered, pos) else {
        vis.set_if_neq(Visibility::Hidden);
        return;
    };
    let Some(stack) = craftable.0.get(slot.0) else { return; };
    let Some(recipe) = recipes.0.iter().find(|r| r.result.id == stack.id) else { return; };

    let mut label = String::new();
    for ing in &recipe.items {
        let name = items.0.get(&ing.id).map_or("?", |d| d.label.as_str());
        label.push_str(&format!("{}x {}\n", ing.count, name));
    }
    let result_name = items.0.get(&stack.id).map_or("?", |d| d.label.as_str());
    label.push_str(&format!("-> {}x {}", recipe.result.count, result_name));

    if text.0 != label { text.0 = label; }
    node.left = px(pos.x + 16.0);
    node.top = px(pos.y + 16.0);
    vis.set_if_neq(Visibility::Visible);
}

pub fn craftable_click(
    mouse: Res<ButtonInput<MouseButton>>,
    slots: Query<(&CraftableSlot, &Interaction)>,
    list_vis: Query<&Visibility, With<CraftableList>>,
    craftable: Res<CraftableItems>,
    recipes: Res<Recipes>,
    items: Res<Items>,
    mut cursor: ResMut<CursorItem>,
    mut inventory: Single<&mut Inventory, With<Player>>,
) {
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    if !left && !right { return; }

    let Ok(vis) = list_vis.single() else { return; };
    if *vis == Visibility::Hidden { return; }

    let Some((slot, _)) = slots.iter().find(|(_, i)| **i != Interaction::None) else { return; };
    let Some(stack) = craftable.0.get(slot.0) else { return; };
    let Some(recipe) = recipes.0.iter().find(|r| r.result.id == stack.id) else { return; };

    // Cursor darf nur dasselbe Item halten
    if let Some(held) = &cursor.0 {
        if held.id != recipe.result.id { return; }
    }

    // 1) Limit durch vorhandene Zutaten
    let by_ingredients = recipe.items.iter()
        .filter(|ing| ing.count > 0)
        .map(|ing| count_items(&inventory, ing.id) / ing.count)
        .min()
        .unwrap_or(0);

    // 2) Limit durch freien Platz im Cursor-Stack
    let Some(def) = items.0.get(&recipe.result.id) else { return; };
    let held_count = cursor.0.as_ref().map_or(0, |h| h.count);
    let free = def.max_stack.saturating_sub(held_count);
    let by_stack = free / recipe.result.count.max(1);

    let wanted = if left { 1 } else { u32::MAX };
    let rounds = wanted.min(by_ingredients).min(by_stack);
    if rounds == 0 { return; }

    for ing in &recipe.items {
        remove_items(&mut inventory, ing.id, ing.count * rounds);
    }

    let gained = rounds * recipe.result.count;
    match &mut cursor.0 {
        Some(held) => held.count += gained,
        None => cursor.0 = Some(ItemStack { id: recipe.result.id, count: gained }),
    }
}