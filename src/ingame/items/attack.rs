use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::ingame::items::structs::*;
use crate::ingame::player::structs::*;

////definition einer waffe
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponDef {// eigentliche waffe mit stats
    base: WeaponBase,
    kind: WeaponKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeaponBase {
    damage: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WeaponKind {
    Dagger(DaggerDef),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DaggerDef {
    distance: f32, //von player mitt zu dieser entfernung in maus richtung eine linie: alles in der linie ist getroffen, in pixeln 
    duration: f32, //zeit von klick über ausgefahrenen dolch zurück zum normalen zustand in sekunden
    hit_at: f32, //zeit punk zwischen 0 - duration, an dem es als physischer hit zählt
    cooldown: f32, //zeit nach duration (animations- ) ende, die gewartet wird bis nächsten angriff
}
////ende waffendefinition


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

pub fn animate_held_item(
    items: Res<Items>,
    attackers: Query<&Attacker>,
    pivots: Query<&ChildOf, With<HandPivot>>,
    mut sprites: Query<(&ChildOf, &mut Transform, &mut Visibility, &mut Sprite), With<HeldItemSprite>>,
    asset_server: Res<AssetServer>,
) {
    for (sprite_parent, mut tf, mut vis, mut sprite) in &mut sprites {
        // Sprite -> HandPivot -> Player
        let Ok(pivot_parent) = pivots.get(sprite_parent.parent()) else { continue; };
        let Ok(attacker) = attackers.get(pivot_parent.parent()) else { continue; };

        let Some(active) = &attacker.active else {
            // kein Angriff: zurücksetzen
            *vis = Visibility::Hidden;
            tf.translation = Vec3::ZERO;
            tf.rotation = Quat::IDENTITY;
            continue;
        };

        let Some(hand_item) = items.0.get(&active.item) else { continue; }; 
        let Some(weapon) = hand_item.weapon.as_ref() else { continue; };

        match &weapon.kind {
            WeaponKind::Dagger(def) => {
                if active.elapsed >= def.duration {
                    *vis = Visibility::Hidden;
                    tf.translation = Vec3::ZERO;
                    tf.rotation = Quat::IDENTITY;
                    continue;
                }
                *vis = Visibility::Visible;
                let handle = asset_server.load(hand_item.asset.clone());
                if sprite.image != handle {
                    sprite.image = handle;
                }


                let t = (active.elapsed / def.duration).clamp(0.0, 1.0);
                // 0 -> 1 -> 0: ausfahren und zurück
                let thrust = (t * std::f32::consts::PI).sin();

                let dir = active.aim.normalize_or_zero();
                tf.translation = (dir * def.distance * thrust).extend(0.0);
                tf.rotation = Quat::from_rotation_z(dir.to_angle() - 45.0_f32.to_radians());
            }
        }
    }
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
        let mut is_over = false;

        //waffenspezifische animation und hit logik...
        match &weapon.kind {
            WeaponKind::Dagger(def) => {
                if active.elapsed >= def.duration + def.cooldown {
                    is_over = true;
                } else if active.hit_fired == true{
                    continue; // wenn hit bereits abgesetzt wurde hier stoppen
                } else {
                    if active.elapsed >= def.hit_at {
                        hits.write(AttackHit {
                            attacker: entity,
                            item: active.item,
                            origin: transform.translation.truncate(),
                            aim: active.aim,
                        });
                        active.hit_fired = true;
                    }
                }                
            }
        }

        if is_over { attacker.active = None; }
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
        info!("hittttt msg");
        // &weapon.kind //hier weiterverwenden
    }
}

