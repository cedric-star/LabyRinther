use bevy::prelude::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::modifier::*;

pub fn apply_click(
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

pub fn can_place(restriction: Option<EquipSlot>, id: ItemId, items: &Items) -> bool {
    restriction.map_or(true, |s| s.accepts(id, items))
}

impl Inventory {
    pub fn row_offset(&self) -> usize { //leere reihen, oben im ui
        MAX_INNER_ROWS.saturating_sub(self.inner_rows.len()) //overflow verhindern, 0
        //return
    }

    //get inv slot by cords
    pub fn slot(&self, x: usize, y: usize) -> Option<&InventorySlot> {
        if y == HOTBAR_ROW {
            return self.hotbar.get(x);
        }
        let r = y.checked_sub(self.row_offset())?; //returnt none bei over/underflow
        self.inner_rows.get(r)?.get(x)
    }

    pub fn slot_mut(&mut self, x: usize, y: usize) -> Option<&mut InventorySlot> {
        if y == HOTBAR_ROW {
            return self.hotbar.get_mut(x);
        }
        let r = y.checked_sub(self.row_offset())?;
        self.inner_rows.get_mut(r)?.get_mut(x)
    }
}

impl Default for Inventory {
    fn default() -> Self {

        let es = InventorySlot { stack: None, };
        let empty_row = [es; 10];
        let mut other_rows = Vec::new();
        other_rows.push(empty_row);
        other_rows.push(empty_row);

        Self {
            hotbar: [
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::WoddenDagger,
                    count: 1,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::Apple,
                    count: 4,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::IronChest,
                    count: 1,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::WoddenDagger,
                    count: 1,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::MediumDamagePotion,
                    count: 5,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::MediumHealthPotion,
                    count: 4,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::MediumMaxHealthOrb,
                    count: 9,
                })},
                InventorySlot {stack: Some(ItemStack {
                    id: ItemId::IncrementInventoryRowsSecret,
                    count: 2,
                })},
                es.clone(),
                es.clone(),
            ],
            selected: 0,
            inner_rows: other_rows,
        }
    }
}


impl EquipSlot {
    pub fn accepts(self, id: ItemId, items: &Items) -> bool {
        let Some(def) = items.0.get(&id) else { return false; };
        let cats = &def.categories;
        match self {
            EquipSlot::Helm         => cats.contains(&Category::Helm),
            EquipSlot::Chest        => cats.contains(&Category::Chest),
            EquipSlot::Legs         => cats.contains(&Category::Legs),
            EquipSlot::Boots        => cats.contains(&Category::Boots),
            EquipSlot::Usable(_)    => cats.contains(&Category::Usable),
            EquipSlot::Secret(_)    => cats.contains(&Category::Secret),
        }
    }
}