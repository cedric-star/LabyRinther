use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::ingame::items::structs::*;


////definition einer waffe
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponDef {
    pub damage: f32,
    pub timing: AttackTiming,
    pub kind: AttackKind,
    #[serde(default)]
    pub fx: Option<AttackFx>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AttackFx {
    Swing(SwingFx),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwingFx {
    pub sheet: String, //sheet mit schwungframes
    pub arc_deg: f32, //schwung winkel
    pub windup: f32,
    #[serde(default)]
    pub swish: Option<SwishFx>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwishFx {
    pub sheet: String,
    pub frames: u32,
    pub offset_along: f32, //abstand zu klinge
    pub range: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttackTiming {
    pub duration: f32, //sekunden pro animation
    pub hit_at: f32, //sekunden, wann treffer zählt
    pub cooldown: f32, //pause nach erstem click bis wieder angegriffen wird
    pub frames: f32, //frames in einer animation
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AttackKind {
    Melee { range: f32, arc_deg: f32 }, //reichweite und kegel, alles im kegel getroffen
    Whip { range: f32 }, //nur reichweite, man kann nur ein bestimmtes tile anklicken
    Projectile { speed: f32, lifetime: f32, asset: String },
}////ende waffendefinition


//zustand eines attacks:
#[derive(Component)]
pub struct Attacker {
    pub active: Option<ActiveAttack>,
    pub cooldown_left: f32,
}

impl Default for Attacker {
    fn default() -> Self {
        Self {
            active: None,
            cooldown_left: 0.0,               
        }
    }
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
    pub origin: Vec2,
    pub aim: Vec2,
}

//wenn enemy oder player linksclick drückt / angreift -> dann event senden, wenn zeit für hit ist
pub fn send_hit_msg(
    time: Res<Time>,
    items: Res<Items>,
    mut hits: MessageWriter<AttackHit>,
    mut q: Query<(Entity, &mut Attacker, &Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut attacker, transform) in &mut q {
        attacker.cooldown_left = (attacker.cooldown_left - dt).max(0.0);

        let Some(active) = attacker.active.as_mut() else { continue; };
        let Some(weapon) = items.0.get(&active.item).and_then(
            |d| d.weapon.as_ref()
        ) else { 
            attacker.active = None;
            continue; 
        };

        active.elapsed += dt;

        if !active.hit_fired && active.elapsed >= weapon.timing.hit_at {
            hits.write(AttackHit {
                attacker: entity,
                item: active.item,
                origin: transform.translation.truncate(),
                aim: active.aim,
            });
            info!("hit!!!");
        }

        if active.elapsed >= weapon.timing.duration {
            attacker.cooldown_left = weapon.timing.cooldown;
            attacker.active = None;
        }


    }
}




//readed messages und reagiert mit entprechendem hit
pub fn resolve_hits(
    mut hits: MessageReader<AttackHit>,
    items: Res<Items>,
    mut commands: Commands,
) {
    for hit in hits.read() {
        let Some(weapon) = items.0.get(&hit.item)
            .and_then(|d| d.weapon.as_ref())
            else { continue; };
        match &weapon.kind {
            AttackKind::Melee { range, arc_deg } => {
                println!("meele hit");
            }
            _ => {
                println!("not melle hit");
            }
        }
    }
}

