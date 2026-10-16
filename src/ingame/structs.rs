use bevy::prelude::*;
use bevy::app::*;
use crate::components::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;
use crate::ingame::player::ui::*;
use crate::ingame::player::util::*;
use crate::ingame::player::systems::*;
use crate::ingame::player::structs::*;
use crate::settings::settings_screen::*;
use crate::ingame::status_bars::*;
use crate::ingame::recipes::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::systems::*;
use crate::ingame::inventory::ui::*;
use crate::ingame::recipes::structs::*;
use crate::ingame::recipes::systems::*;
use crate::ingame::recipes::ui::*;

#[derive(Component)]
pub struct IngameUi;

#[derive(Component)]
pub struct HotBar;

#[derive(Component)]
pub struct HotBarSelector;

#[derive(Component, Debug)]
pub struct HotBarItem(pub usize);

#[derive(Component)]
pub struct HotBarItemNum(pub usize);
