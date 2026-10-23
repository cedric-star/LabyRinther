use bevy::prelude::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::modifier::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::player::structs::*;

// 1) Equipped-Effekte neu aufbauen
pub fn rebuild_equipped_stats(
    items: Res<Items>,
    mut q: Query<(&Equipment, &mut Stats), Changed<Equipment>>,
) {
    for (equipment, mut stats) in &mut q {
        stats.equipped.clear();
        for stack in equipment.iter_stacks() {
            let Some(def) = items.0.get(&stack.id) else { continue };
            for e in def.effects.iter().filter(|e| e.trigger == EffectTrigger::Equipped) {
                stats.equipped.push((e.stat, e.modifier));
            }
            if let Some(a) = &def.armor { 
                stats.equipped.push((Stat::Armor, Modifier::Add(a.points))); 
            }
        }
    }
}

// 2) Caches in Health/Mana synchronisieren
pub fn sync_vitals(mut q: Query<(&Stats, &mut Health, &mut Mana), Changed<Stats>>) {
    for (stats, mut hp, mut mana) in &mut q {
        hp.max_health = stats.get(Stat::MaxHealth);
        hp.health_points = hp.health_points.min(hp.max_health);
        mana.max_mana = stats.get(Stat::MaxMana);
        mana.mana_points = mana.mana_points.min(mana.max_mana);
    }
}

// 3) Inventarreihen anpassen
pub fn sync_inventory_rows(mut q: Query<(&Stats, &mut Inventory), Changed<Stats>>) {
    for (stats, mut inv) in &mut q {
        let target = (stats.get(Stat::InventoryRows).round() as usize).min(MAX_INNER_ROWS);
        while inv.inner_rows.len() < target {
            inv.inner_rows.push([InventorySlot::default(); 10]);
        }
        // nur leere Reihen am Ende entfernen
        while inv.inner_rows.len() > target
            && inv.inner_rows.last().is_some_and(|r| r.iter().all(|s| s.stack.is_none()))
        {
            inv.inner_rows.pop();
        }
    }
}
