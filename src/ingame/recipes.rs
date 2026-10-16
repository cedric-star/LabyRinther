use bevy::prelude::*;
use crate::constants::*;
use crate::ingame::inventory::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ingame::inventory_meta::*;
use crate::ingame::items::*;
use std::fs;
use crate::components::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::*;
use crate::ingame::ui::*;
use crate::util::ui::constants::*;
use crate::ingame::inventory::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub items: Vec<ItemStack>, //item + anzahl des items
    pub station: Option<CraftStation>,
    pub result: ItemStack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CraftStation {
    Workbench,
}

#[derive(Resource)]
pub struct Recipes(Vec<Recipe>);

impl Default for Recipes {
    fn default() -> Self {
        let path = "./recipes.json";
        let file = match fs::read_to_string(path) {
            Ok(file) => {
                if file.trim().is_empty() { panic!("recipesjson emtpy):"); }
                file
            }
            Err(e) => { panic!("recipes json nicht ladbar :()"); }
        };

        match serde_json::from_str::<Vec<Recipe>>(&file) {
            Ok(list) => Self(list),
            Err(e) => {panic!("json recipes ungültig )):::")}
        }
    }
}

impl Recipe {

    //none, wenn nicht craftable, ansonsten anzahl, wie oft craftable
    pub fn is_craftable(&self, player_items: HashMap<ItemId, u32>) -> Option<u32> {
        let mut counts: Vec<u32> = Vec::new();

        for recipe_item in self.items.clone() {
            match player_items.get(&recipe_item.id) {
                Some(count) => {
                    if *count < recipe_item.count { return None; }
                    let num_2_craft = *count / recipe_item.count;
                    counts.push(num_2_craft);
                }
                None => { return None; }
            }
        }
        counts.into_iter().map(|c| c * self.result.count).min() //diese funktion is so rust core
    }
}

#[derive(Resource)]
pub struct CraftableItems(Vec<ItemStack>);

impl Default for CraftableItems {
    fn default() -> Self {
        Self(Vec::new())
    }
}

//liste enthält alle craftable items, und wie viele jeweils
pub fn set_craftable_items(
    player: Query<&Player>,
    mut craftable_items: ResMut<CraftableItems>,
    recipes: Res<Recipes>,
) {
    let Ok(player) = player.single() else {
        panic!("didnt found player coreclty");
    };

    let mut player_items: Vec<ItemStack> = Vec::new();
    for inv_rows in player.inventory.inner_rows.clone() {
        for slot in inv_rows {
            if let Some(slot_unwrapped) = slot.stack {
                player_items.push(slot_unwrapped);
            }
        }
    }
    for slot in player.inventory.hotbar {
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

#[derive(Component)]
pub struct CraftableList;          // eindeutiger Marker für den Container

#[derive(Component)]
pub struct CraftableSlot(pub usize); // index in CraftableItems.0

#[derive(Component)]
pub struct CraftableCountText;

#[derive(Component)]
pub struct CraftableTooltip;

pub fn get_craftable_item_field(
    s: &GameSettings,
    player: &Player,
    craftable_items: &CraftableItems
) -> impl Bundle {
    (
        IngameUi,
        GameEntity,
        InventoryShow,
        CraftableList,          // <-- neu
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(100.0),
            grid_column: GridPlacement::start(1),
            grid_row: GridPlacement::start(2),
            flex_wrap: FlexWrap::Wrap,          // slots umbrechen lassen
            align_content: AlignContent::FlexStart,
            padding: UiRect::all(px(8.0)),
            ..default()
        },
        Visibility::Hidden,
        BackgroundColor(BG_COL_A),
    )
}
//helper
//
fn merge_stack(items: Vec<ItemStack>) -> HashMap<ItemId, u32> {
    let mut map: HashMap<ItemId, u32> = HashMap::new();

    for item in items {
        *map.entry(item.id).or_insert(0) += item.count;
    }
    return map
}

//
//
//

pub fn sync_craftable_ui(
    mut commands: Commands,
    craftable: Res<CraftableItems>,
    items: Res<Items>,
    asset_server: Res<AssetServer>,
    s: Res<GameSettings>,
    list: Query<Entity, With<CraftableList>>,
    mut slots: Query<(Entity, &CraftableSlot, &mut ImageNode, &Children)>,
    mut counts: Query<(&mut Text, &mut Visibility), With<CraftableCountText>>,
    mut last: Local<Vec<ItemStack>>,
) {
    // nur arbeiten, wenn sich die Liste wirklich geändert hat
    if *last == craftable.0 { return; }
    *last = craftable.0.clone();

    let Ok(list_entity) = list.single() else { return; };

    // vorhandene slots: bild + zahl aktualisieren
    let mut existing: HashMap<usize, Entity> = HashMap::new();
    for (entity, slot, mut image, children) in &mut slots {
        existing.insert(slot.0, entity);
        let Some(stack) = craftable.0.get(slot.0) else { continue; };

        let handle = asset_server.load(items.0.get(&stack.id).unwrap().asset.clone());
        if image.image != handle { image.image = handle; }

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

pub fn craftable_hover(
    craftable: Res<CraftableItems>,
    recipes: Res<Recipes>,
    items: Res<Items>,
    slots: Query<(&CraftableSlot, &Interaction, &Children)>,
    mut tooltips: Query<(&mut Text, &mut Visibility), With<CraftableTooltip>>,
) {
    for (slot, interaction, children) in &slots {
        let hovered = *interaction == Interaction::Hovered;
        for child in children.iter() {
            let Ok((mut text, mut vis)) = tooltips.get_mut(child) else { continue; };
            if !hovered {
                *vis = Visibility::Hidden;
                continue;
            }
            let Some(stack) = craftable.0.get(slot.0) else { continue; };
            let Some(recipe) = recipes.0.iter().find(|r| r.result.id == stack.id) else { continue; };

            let mut label = String::new();
            for ing in &recipe.items {
                label.push_str(&format!("{}x {}\n", ing.count, items.0.get(&ing.id).unwrap().label));
            }
            label.push_str(&format!("-> {}", items.0.get(&stack.id).unwrap().label));

            if text.0 != label { text.0 = label; }
            *vis = Visibility::Visible;
        }
    }
}

pub fn craftable_click(
    mouse: Res<ButtonInput<MouseButton>>,
    slots: Query<(&CraftableSlot, &Interaction)>,
    list_vis: Query<&Visibility, With<CraftableList>>,
    craftable: Res<CraftableItems>,
    recipes: Res<Recipes>,
    mut cursor: ResMut<CursorItem>,
    mut player: Query<&mut Player>,
) {
    let left = mouse.just_pressed(MouseButton::Left);
    let right = mouse.just_pressed(MouseButton::Right);
    if !left && !right { return; }

    let Ok(vis) = list_vis.single() else { return; };
    if *vis == Visibility::Hidden { return; }
    if cursor.0.is_some() { return; } // schwebendes Item blockiert craften

    let Some((slot, _)) = slots.iter().find(|(_, i)| **i != Interaction::None) else { return; };
    let Some(stack) = craftable.0.get(slot.0) else { return; };
    let Some(recipe) = recipes.0.iter().find(|r| r.result.id == stack.id) else { return; };
    let Ok(mut player) = player.single_mut() else { return; };

    // stack.count = GESAMTE craftbare Menge (runden * result.count)
    let max_rounds = stack.count / recipe.result.count;
    let rounds = if left { 1 } else { max_rounds };
    if rounds == 0 { return; }

    let inv = &mut player.inventory;
    for ing in &recipe.items {
        remove_items(inv, ing.id, ing.count * rounds);
    }

    cursor.0 = Some(ItemStack {
        id: recipe.result.id,
        count: rounds * recipe.result.count,
    });
}

// zieht `amount` items des typs aus hotbar + inneren reihen
fn remove_items(inv: &mut Inventory, id: ItemId, mut amount: u32) {
    let mut take_from = |slot: &mut InventorySlot| {
        if amount == 0 { return; }
        if let Some(s) = &mut slot.stack {
            if s.id == id {
                let take = s.count.min(amount);
                s.count -= take;
                amount -= take;
                if s.count == 0 { slot.stack = None; }
            }
        }
    };
    for slot in inv.hotbar.iter_mut() { take_from(slot); }
    for row in inv.inner_rows.iter_mut() {
        for slot in row.iter_mut() { take_from(slot); }
    }
}
