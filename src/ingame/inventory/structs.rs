use bevy::prelude::*;
use crate::ingame::items::structs::*;


#[derive(Component)]
pub struct InventoryRoot;

#[derive(Component)]
pub struct InventoryShow;

#[derive(Component)]
pub struct InventorySlotNormalCords(pub usize, pub usize);

#[derive(Component)]
pub struct InventorySlotItemNormalCords(pub usize, pub usize);

#[derive(Component)]
pub struct InventorySlotUsablesCords(pub usize, pub usize);

#[derive(Component, Debug)]
pub struct InventorySlotEquip(pub EquipSlot);

#[derive(Component)]
pub struct InventorySlotSecret(pub usize);

#[derive(Component)]
pub struct InventoryPlayerView;

#[derive(Component)]
pub struct InventorySlotCount(pub usize, pub usize);

#[derive(Component)]
pub struct CursorItemVisual;

#[derive(Component)]
pub struct CursorCountText;

#[derive(Resource, Default)]
pub struct CursorItem(pub Option<ItemStack>);

#[derive(Component)]
pub struct InventoryEquipItem(pub EquipSlot);

#[derive(Component)]
pub struct CursorItemInfoText; //wenn im inventar über ein item gehovered wird, soll ein test erscheinen, der inofs anzeigt


#[derive(Debug, Component)]
pub struct Inventory {
    pub hotbar: [InventorySlot; 10],
    pub selected: usize,
    pub inner_rows: Vec<[InventorySlot; 10]>,
}



#[derive(Clone, Debug, Default, Copy)]
pub struct InventorySlot {
    pub stack: Option<ItemStack>,
}

pub const MAX_INNER_ROWS: usize = 6;
pub const HOTBAR_ROW: usize = MAX_INNER_ROWS;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)] //nur manche items in inventar-slots
pub enum EquipSlot { 
    Helm,
    Chest, 
    Legs, 
    Boots, 
    Usable(usize), 
    Secret(usize) 
}

