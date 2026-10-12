use bevy::prelude::*;
use crate::constants::*;
use std::collections::HashSet;

#[derive(Debug, Component)]
pub struct Inventory {
    pub hotbar: [InventorySlot; 10],
    pub selected: usize,
    pub inner_rows: Vec<[InventorySlot; 10]>,
    pub equipment: Equipment,
}

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub helm: Option<ItemStack>,
    pub chest: Option<ItemStack>,
    pub legs: Option<ItemStack>,
    pub boots: Option<ItemStack>,
    pub usable: Option<ItemStack>,
    pub secret: Option<ItemStack>,
}

#[derive(Clone, Debug, Default, Copy)]
pub struct InventorySlot {
    pub stack: Option<ItemStack>,
}

#[derive(Clone, Debug, Copy)]
pub struct ItemStack {
    pub id: ItemId,
    pub count: u32,
}

#[derive(Clone, Default, Debug, Component)]
pub struct Item {
    pub stack_size: u32,
    pub item: ItemDef,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ItemId {
    IronChest,
    Apple,
    JucyWoddenDagger,
    WoddenDagger,
    #[default]
    Thing,
    Nothing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Resource,
    Armor,
    Helm,
    Chest,
    Legs,
    Boots,
    Weapon,
    Tool,
    Consumable,
    Usable,
    MazeUtil,
    Food,
    Waste,
    Secret,
    Nothing,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    #[default]
    F,
}

#[derive(Clone, Debug, Default)]
pub struct ItemDef {
    pub id: ItemId,
    pub label: &'static str,
    pub asset: &'static str,
    pub categories: &'static [Category],
    pub tier: Tier,
    pub max_stack: u32,
    //optionale attribute:
    pub weapon: Option<WeaponDef>,
    pub armor: Option<ArmorDef>,
}

#[derive(Clone, Copy, Debug)]
pub struct WeaponDef {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub wtype: WeaponType,
}

#[derive(Clone, Copy, Debug)]
pub struct ArmorDef {
    pub points: f32, //max 15 bitti
}

#[derive(Clone, Copy, Debug)]
pub enum WeaponType {
    Sword,
    Staff,
    Shooting,
    Whip,
}

impl Tier {
    //in prozent
    pub fn chance(self) -> f32 {
        match self {
            Tier::S => { 0.1 }
            Tier::A => { 0.9 }
            Tier::B => { 2.0 }
            Tier::C => { 5.0 }
            Tier::D => { 30.0 }
            Tier::F => { 60.0 }
        }
    }
}

pub struct Recipe {
    pub items: &'static [(ItemId, i32)], //item + anzahl des items
    pub station: Option<CraftStation>,
}

pub struct CraftStation;

pub const RECIPES: &[Recipe; 1] = &[
    Recipe {
        items: &[
            (ItemId::WoddenDagger, 1),
            (ItemId::Apple, 2),
        ],
        station: None,
    },
];

pub const ITEMS: &[ItemDef; 5] = &[
    ItemDef {
        id: ItemId::Nothing,
        label: "Error",
        asset: "items/Error.png",
        categories: &[Category::Nothing],
        tier: Tier::F,
        max_stack: 1,
        weapon: None,
        armor: None,
    },
    ItemDef {
        id: ItemId::IronChest,
        label: "Iron Chest",
        asset: "items/IronChest.png",
        categories: &[Category::Armor, Category::Chest],
        tier: Tier::D,
        max_stack: 1,
        weapon: None,
        armor: Some(ArmorDef {
            points: 5.0,
        })
    },
    ItemDef {
        id: ItemId::Apple,
        label: "Apple",
        asset: "items/Apple.png",
        categories: &[Category::Food, Category::Resource],
        tier: Tier::F,
        max_stack: 999,
        weapon: None,
        armor: None,
    },
    ItemDef {
        id: ItemId::WoddenDagger,
        label: "Wodden Dagger",
        asset: "items/WoddenDagger.png",
        categories: &[Category::Resource, Category::Weapon],
        tier: Tier::F,
        max_stack: 1,
        weapon: Some(WeaponDef {
            damage: 10.0,
            range: 2.0,
            attack_speed: 1.0,
            wtype: WeaponType::Sword,
        }),
        armor: None,
    },
    ItemDef {
        id: ItemId::JucyWoddenDagger,
        label: "Jucy Wodden Dagger",
        asset: "items/JucyWoddenDagger.png",
        categories: &[Category::Weapon],
        tier: Tier::D,
        max_stack: 1,
        weapon: Some(WeaponDef {
            damage: 12.0,
            range: 2.0,
            attack_speed: 1.5,
            wtype: WeaponType::Sword,
        }),
        armor: None,
    },
];

pub fn get_item_by_id(id: ItemId) -> ItemDef {
    for item in ITEMS {
        if item.id == id {
            return item.clone();
        }
    }
    return ITEMS[0].clone();
}

pub const MAX_INNER_ROWS: usize = 6;
pub const HOTBAR_ROW: usize = MAX_INNER_ROWS;

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

    pub fn equip_mut(&mut self, s: EquipSlot) -> &mut Option<ItemStack> {
        match s {
            EquipSlot::Helm     => &mut self.equipment.helm,
            EquipSlot::Chest    => &mut self.equipment.chest,
            EquipSlot::Legs     => &mut self.equipment.legs,
            EquipSlot::Boots    => &mut self.equipment.boots,
            EquipSlot::Usable   => &mut self.equipment.usable,
            EquipSlot::Secret   => &mut self.equipment.secret,
        }
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
                es.clone(),
                es.clone(),
                es.clone(),
                es.clone(),
                es.clone(),
                es.clone(),
            ],
            selected: 0,
            equipment: Equipment::default(),
            inner_rows: other_rows,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)] //nur manche items in inventar-slots
pub enum EquipSlot { Helm, Chest, Legs, Boots, Usable, Secret }

impl EquipSlot {
    pub fn accepts(self, id: ItemId) -> bool {
        let cats = get_item_by_id(id).categories;
        match self {
            EquipSlot::Helm     => cats.contains(&Category::Helm),
            EquipSlot::Chest    => cats.contains(&Category::Chest),
            EquipSlot::Legs     => cats.contains(&Category::Legs),
            EquipSlot::Boots    => cats.contains(&Category::Boots),
            EquipSlot::Usable   => cats.contains(&Category::Usable),
            EquipSlot::Secret   => cats.contains(&Category::Secret),
        }
    }
}

pub fn can_place(restriction: Option<EquipSlot>, id: ItemId) -> bool {
    restriction.map_or(true, |s| s.accepts(id))
}
