use bevy::prelude::*;
use crate::constants::*;
use crate::ingame::inventory::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::ingame::items::*;
use std::fs;
use crate::components::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::ui::*;
use crate::ingame::player::util::*;
use crate::ingame::player::systems::*;
use crate::ingame::player::structs::*;
use crate::ingame::ui::*;
use crate::util::ui::constants::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::systems::*;
use crate::ingame::inventory::ui::*;
use crate::ingame::recipes::structs::*;
use crate::ingame::recipes::systems::*;
use crate::ingame::recipes::util::*;
use crate::ingame::util::*;
use crate::ingame::util::*;
use crate::ingame::systems::*;
use crate::ingame::ui::*;
use crate::ingame::structs::*;

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