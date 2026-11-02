use bevy::prelude::*;
use bevy::math::bounding::*;
use serde::{Serialize, Deserialize};
use crate::ingame::items::structs::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::attack::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::attack::util::*;
use crate::constants::*;

pub fn animate_held_item(
    items: Res<Items>,
    attackers: Query<&Attacker>,
    pivots: Query<&ChildOf, With<HandPivot>>,
    mut sprites: Query<(
        &ChildOf,
        &mut Transform,
        &GlobalTransform,
        &mut Visibility,
        &mut Sprite,
        &Hitbox,
    ), With<HeldItemSprite>>,
    asset_server: Res<AssetServer>,
    mut gizmos: Gizmos,
) {
    for (sprite_parent, mut tf, gt, mut vis, mut sprite, hitbox) in &mut sprites {
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

        hitbox.draw(&mut gizmos, gt.compute_transform());

        match &weapon.kind {
            WeaponKind::Dagger(def) => {
                // wenn elapsed für animation vorbei ist, animation fertig -> abbruch
                if active.elapsed >= def.duration {
                    *vis = Visibility::Hidden;
                    tf.translation = Vec3::ZERO;
                    tf.rotation = Quat::IDENTITY;
                    continue;
                }

                //sprite setzen und sichtbar machen
                *vis = Visibility::Visible;
                let handle = asset_server.load(hand_item.asset.clone());
                if sprite.image != handle {
                    sprite.image = handle;
                }

                // elapsed/duration -> prozent, .clamp macht alles < zu 0 und > zu 1.
                let t = (active.elapsed / def.duration).clamp(0.0, 1.0);
                
                //t = 0-1 -> 0/1 pi = 0, alles dazwischen wird zu einer sinuskurve (sinus rechnet mit pi werten)
                let thrust = (t * std::f32::consts::PI).sin();

                //einheitsvektor (länge 1) der richtung von hand aus
                let dir = active.aim.normalize_or_zero();

                //richtungsvektor * länge = vektor mit korekter richtung und könge
                // * thrust verschiebt vektorurpsrung auf sinuskurve
                //extend macht aus vec2 einen vec3 mit z=0
                tf.translation = (dir * def.distance * thrust).extend(0.0);

                // to_angle() macht aus vec2 einen radiant winkel
                // - 45 wegen asset drehung von daggern
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
    mut q: Query<(Entity, &mut Attacker, &Hitbox, &Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut attacker, hitbox, transform) in &mut q {
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
                            origin: hitbox.center(transform.translation.truncate()),
                            aim: active.aim.normalize_or_zero(),
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
//nicht bei treffer, sondern bei angriff ausgelöst -> prüft ob treffer
pub fn resolve_hits(
    mut hits: MessageReader<AttackHit>,
    items: Res<Items>,
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Health, &mut Transform, &Hitbox)>,
) {
    for hit in hits.read() {
        let Some(weapon) = items.0.get(&hit.item)
            .and_then(|d| d.weapon.as_ref())
            else { continue; };
        match &weapon.kind {
            WeaponKind::Dagger(dagger_def) => {
                for (entity, mut health, mut transform, hitbox) in enemies.iter_mut() {
                    if hit.attacker == entity { continue; }
                    resolve_dagger_attack(
                        &mut health, 
                        transform.translation.truncate(), 
                        hitbox,
                        hit,
                        dagger_def,
                    );
                }
            }
            //weitere waffentyppen
        }
    }
}


pub fn resolve_dagger_attack(
    health: &mut Health,
    target_pos: Vec2,
    target_box: &Hitbox,
    attack_hit: &AttackHit,
    dagger: &DaggerDef,
) {
    let Ok(dir) = Dir2::new(attack_hit.aim) else { return };

    // Optional: Strahl "dicker" machen, indem die Zielbox aufgeblasen wird
    const THICKNESS: f32 = 3.0;
    let mut aabb = target_box.aabb(target_pos);
    aabb.min -= Vec2::splat(THICKNESS);
    aabb.max += Vec2::splat(THICKNESS);

    let ray = RayCast2d::new(attack_hit.origin, dir, dagger.distance);
    if ray.aabb_intersection_at(&aabb).is_some() {
        println!("HIT");
        // health.health_points -= ...
    }
}
