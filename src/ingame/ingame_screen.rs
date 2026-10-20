use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use crate::world::world_generator::*;
use crate::components::*;
use crate::ingame::player::systems::*;
use crate::ingame::player::structs::*;
use crate::ingame::ui::*;
use crate::ingame::systems::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::inventory::systems::*;
use crate::ingame::status_bars::*;
use crate::ingame::recipes::structs::*;
use crate::ingame::recipes::systems::*;

pub fn ingame_plugin(app: &mut App) {
    app
        .init_resource::<CursorItem>()
        .init_resource::<CraftableItems>()
        .add_systems(
            OnEnter(GameState::InGame),
            (
                startup.run_if(not(any_with_component::<Player>)),
                spawn_ui,
            ).chain(),

        )
        .add_systems(
            OnExit(GameState::InGame),
            cleanup_ui,
        )
        .add_systems(OnEnter(GameState::Welcome), cleanup_game)
        .add_systems(
            FixedUpdate,
            (controls, load_map).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (
                sync_craftable_ui,
                craftable_hover,
                craftable_click,
                set_hotkeys,
                update_bars,
                movement,
                update_ui,
                update_inventory_items,
                inventory_click.before(update_inventory),
                update_inventory,
                update_cursor_visual,
                update_inventory_info_text,
                update_equipment,
                mov_cam,
                mana_regen.run_if(on_timer(Duration::from_secs(1))),
                health_regen.run_if(on_timer(Duration::from_secs(1))),
                set_craftable_items.run_if(on_timer(Duration::from_millis(200))),
            ).run_if(in_state(GameState::InGame)),
        );
}
