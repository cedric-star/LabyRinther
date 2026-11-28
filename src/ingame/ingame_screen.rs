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
use crate::ingame::items::systems::*;
use crate::ingame::items::attack::systems::*;
use crate::ingame::animation::*;
use crate::ingame::entity::systems::*;
use crate::ingame::items::attack::structs::*;
use crate::ingame::entity::move_enemy::*;
use crate::ingame::entity::spawn_enemy::*;
use crate::ingame::entity::enemy_waves::*;
use crate::net::{enemy_spawn_ready, tick_enemy_spawn_timer, has_authority};

pub fn ingame_plugin(app: &mut App) {
    app
        .init_resource::<CursorItem>()
        .init_resource::<CraftableItems>()
        .add_message::<AttackHit>()

        //---------
        //startup
        .add_systems(OnEnter(GameState::InGame),(
            startup.run_if(not(any_with_component::<Player>)),//nur wenn player nicht schon existiert
            spawn_ui,
        ).chain())
        //---------

        //---------
        //beim exit aus ingame
        .add_systems(OnExit(GameState::InGame),
        cleanup_ui,
        )

        //wenn in welcome gegangen wird, alles löschen/aufräumen
        .add_systems(OnEnter(GameState::Welcome),
         cleanup_game
        )
        //---------

        //---------
        //fixed update: controls, alles was fest oft durchlaufen wird
       .add_systems(
            FixedUpdate,
            (
                controls, //spieler mit wasd bewegen
                load_map, //map laden (chunks, ...) für player, um chunksize
                move_enemies.run_if(has_authority), // NEU: nur Host/Singleplayer simuliert Gegner
            ).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (
                set_enemy_movement.run_if(has_authority), // NEU
                move_enemies.run_if(has_authority),      // NEU
            ).chain().run_if(in_state(GameState::InGame)),
        )
        //---------

        //---------
        // normale updates

        // ui für player
        .add_systems(
            Update,
            (
                //crafting
                set_craftable_items.run_if(on_timer(Duration::from_millis(200))),
                craftable_click,

                //items im ui anhand von informationen anzeigen
                update_cursor_visual,
                update_equipment_ui,
                sync_craftable_ui,
                update_inventory,
                update_hotbar_items_ui,

                //tooltips
                update_inventory_info_text,
                update_euqipped_info_text,
                craftable_hover,

                //andere
                set_hotkeys,
                update_selected_hotbar_slot_ui,
                inventory_click.before(update_inventory),
                mov_cam,
            ).run_if(in_state(GameState::InGame))
        )

        //health/mana/armor systeme
        .add_systems(
            Update,
            (
                update_bars,
                mana_regen.run_if(on_timer(Duration::from_secs(1))),
                health_regen.run_if(on_timer(Duration::from_secs(1))),
                despawn_enemy_by_health.run_if(has_authority), // NEU: Tod von Gegnern entscheidet der Host
            ).run_if(in_state(GameState::InGame)),
        )

        //item interaktionen/bezogene dinge /(und waffen)
        .add_systems(
            Update,
            (
                use_item_system,
                weapon_use,
            ).run_if(in_state(GameState::InGame)),
        )

        //enemy spezifische systeme
        .add_systems(
            Update,
            (
                set_enemy_player_seen.run_if(has_authority), 
                set_waves.run_if(has_authority),        
                spawn_enemy_near_player.run_if(enemy_spawn_ready),
                tick_enemy_spawn_timer,
                draw_enemy_health_bar, // läuft überall: Geister haben Health + EnemyHealthBar
            ).run_if(in_state(GameState::InGame)),
        )

        //entity normale sachen (animationen, attacks)
        .add_systems(
            Update,
            (
                switch_clip,
                animate_entity,
                animate_held_item,
                set_attacker,
            ).run_if(in_state(GameState::InGame)),
        )

        //stats/modifier spezifische systeme (equipment -> stat setzen)
        .add_systems(
            Update,
            (
                rebuild_equipped_stats,
                sync_vitals,
                sync_inventory_rows,
            ).run_if(in_state(GameState::InGame)),
        )
        //---------

        //---------
        //nach normalen updates

        //angriff/schaden
        .add_systems(
            PostUpdate,
            // NEU: Treffer/Schaden wird nur vom Host berechnet
            weapon_contact.after(TransformSystems::Propagate).run_if(has_authority),
        );
        //---------
}