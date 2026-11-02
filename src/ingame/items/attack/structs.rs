use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::ingame::items::structs::*;
use crate::ingame::player::structs::*;

////definition einer waffe
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponDef {// eigentliche waffe mit stats
    pub base: WeaponBase,
    pub kind: WeaponKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponBase {
    pub damage: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WeaponKind {
    Dagger(DaggerDef),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DaggerDef {
    pub distance: f32, //von player mitt zu dieser entfernung in maus richtung eine linie: alles in der linie ist getroffen, in pixeln 
    pub duration: f32, //zeit von klick über ausgefahrenen dolch zurück zum normalen zustand in sekunden
    pub hit_at: f32, //zeit punk zwischen 0 - duration, an dem es als physischer hit zählt
    pub cooldown: f32, //zeit nach duration (animations- ) ende, die gewartet wird bis nächsten angriff
}
////ende waffendefinition


//zustand eines attacks:
#[derive(Component)]
pub struct Attacker {
    pub active: Option<ActiveAttack>,
    pub cooldown_left: f32,
}

pub struct ActiveAttack {
    pub item: ItemId,
    pub elapsed: f32,
    pub hit_fired: bool,
    pub aim: Vec2,        // Richtung beim Start einfrieren
}//attack zustand ende


//löst dann aus, wenn hit_at in attack_timing erreicht wird und speichert wichtige infos
//message die gesetzt und ausgelesen wird
#[derive(Message)] 
pub struct AttackHit {
    pub attacker: Entity,
    pub item: ItemId,
    pub origin: Vec2, //trefferkoordinate ursprung in px
    pub aim: Vec2,    //richtungsvektor: kreis um origin, darauf ein punkt
}