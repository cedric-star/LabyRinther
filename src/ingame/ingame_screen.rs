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

pub fn ingame_plugin(app: &mut App) {
    app
        .init_resource::<CursorItem>()
        .init_resource::<CraftableItems>()
        .add_message::<AttackHit>()

        //---------
        //startup
        .add_systems(OnEnter(GameState::InGame),(
            startup.run_if(not(any_with_component::<Player>)),//nur wennn player nicht schon existiert
            spawn_ui,
        ).chain())
        //---------

        //---------
        //beim exit aus ingame
        .add_systems(OnExit(GameState::InGame),
        cleanup_ui,
        )

        //wenn in welcom gegangen wird, alles löschen/aufräumen
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
                move_enemies, //gegner automatisch bewegen (einfach rumlaufen oder verfolgen, ...)
            ).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (
                set_enemy_movement, //zuerst enemy movement setzen (normales umherlaufen), dann laufen (wegen kollisionen)
                move_enemies, //gegner automatisch bewegen (einfach rumlaufen oder verfolgen, ...)
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
                set_craftable_items.run_if(on_timer(Duration::from_millis(200))), //craftable items berechnen
                craftable_click, //erkennt klicks im craft menü -> einzelne items/alle craften und in schwebende ansich heben

                //items im ui anhand von informationen anzeigen
                update_cursor_visual, //aktualisiert schwebende anzeige im ui -> item sprite setzen
                update_equipment_ui, //zeigt items aus euqipment inventar im ui an
                sync_craftable_ui, //zeigt craftable items links im ui, anhand verfügbarer items im inventar
                update_inventory,//items im inventar im ui anzeigen
                update_hotbar_items_ui, //zeigt items im ui aus hotbar an 

                //tooltips
                update_inventory_info_text, //tooltip für items aus normalen inventar slots
                update_euqipped_info_text, //tooltip für items aus equipment slots
                craftable_hover, //wenn über craftable item gehovered -> tooltip anzeigen 

                //andere
                set_hotkeys, //tastenerennung für 0-9 und setzen hotkeys + und e für inventar öffnen     
                update_selected_hotbar_slot_ui, //ausgewählten hotbarslot im ui anzeigen
                inventory_click.before(update_inventory), //erkennt klick in inventar -> sschwebende anzeige
                mov_cam, //kamera soll player verfolgen
            ).run_if(in_state(GameState::InGame))
        )

        //health/mana/armor systeme
        .add_systems(
            Update,
            (
                update_bars, //bars oben rechts in ecke: mana, health, armor
                mana_regen.run_if(on_timer(Duration::from_secs(1))), //mana regenerieren
                health_regen.run_if(on_timer(Duration::from_secs(1))), //health regenerieren
                despawn_enemy_by_health, //gegner weg, wenn tot

            ).run_if(in_state(GameState::InGame)),
        )

        //item interaktionen/bezogene dinge /(und waffen)
        .add_systems(
            Update,
            (
                use_item_system, //wenn ein item ausgewählt und benutzt wird -> aktion durchführen
                weapon_use, //wenn weapon geklickt
            ).run_if(in_state(GameState::InGame)),
        )

        //enemy spezifische systee
        .add_systems(
            Update,
            (
                set_enemy_player_seen, //überwacht enemies und esetzt wenn player gesehen wurde
                set_waves, //überwacht waves und setzt aktiv/inaktiv
                spawn_enemy_near_player.run_if(on_timer(Duration::from_millis(200))), //gegner jede sekunde spawnen
                draw_enemy_health_bar, //malt enemy health bar, anhand der health
            ).run_if(in_state(GameState::InGame)),
        )

        //entity normale sachen (animationen, attacks)
        .add_systems(
            Update,
            (   
                switch_clip, //überwacht animationsstate und setzt entsprechende animationssprites 
                animate_entity, //schaltet frames aus spriteatlas für animationen zeitbasiert weiter
                animate_held_item, //animiert sprite in hand des attackers & setzt animationsinfos im struct für waffentypspezifische angriffe
                set_attacker, //aktiven angriff zeitbasiert setzen/beenden
            ).run_if(in_state(GameState::InGame)),
        )

 
        //stats/modifier spezifische systeme (equipment -> stat setzen)
        .add_systems(
            Update,
            (
                rebuild_equipped_stats, //geht equipped items in inventar durch und setzt stats (modifier)
                sync_vitals, //setzt maximales mana/health anhand von stats
                sync_inventory_rows, //inventarreihenanzahl anhand von stats setzen
                
            ).run_if(in_state(GameState::InGame)),
        )
        //---------

        //---------
        //nach nocrmalen updates
        
        //angriff/schaden
        .add_systems(
            PostUpdate,
            weapon_contact.after(TransformSystems::Propagate), //erkennt hit und verteilt schaden/knockback,...
        );
        //---------
}
