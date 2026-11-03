use bevy::prelude::*;
use bevy::math::bounding::*;
use crate::ingame::items::structs::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::attack::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::attack::util::*;

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

//setzt relevante variablen des attacker wie zeit, oder ob angriff vorbei ist
pub fn set_attacker(
    time: Res<Time>,
    items: Res<Items>,
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
                        
                        active.hit_fired = true;
                    }
                }                
            }
        }

        if is_over { attacker.active = None; }
    }
}


//Prüft jeden Frame, ob die (gedrehte) Hitbox einer geführten Waffe
// die Hitbox eines Ziels überschneidet, und verteilt dann einmalig Schaden.
pub fn weapon_contact(
    items: Res<Items>,
    // Waffen-Sprites: Weltposition+Rotation, lokale Box, Sichtbarkeit, Parent (= HandPivot/item in hand)
    held_weapons: Query<(&GlobalTransform, &Hitbox, &Visibility, &ChildOf), With<HeldItemSprite>>,
    hand_pivots: Query<&ChildOf, With<HandPivot>>, //nur für den parent wichtig
    mut attackers: Query<&mut Attacker>, //alle angreifer
    mut targets: Query<(Entity, &mut Health, &Transform, &Hitbox), With<Hittable>>, //alle potentiellen ziele des angriffs
) {
    for (weapon_global_tf, weapon_hitbox, weapon_visibility, weapon_parent) in &held_weapons {
        if *weapon_visibility == Visibility::Hidden { continue; } //keine waffe -> kein angriffh-´

        let Ok(pivot_parent) = hand_pivots.get(weapon_parent.parent()) else { continue; };
        let owner_entity = pivot_parent.parent(); // wer die waffe führt darf sich nicht selbst treffen
        // Angriffszustand des Besitzers holen
        let Ok(mut attacker) = attackers.get_mut(owner_entity) else { continue; };

        // `active` = der gerade laufende Angriff (Option<ActiveAttack>):
        // enthält Item, vergangene Zeit, Richtung und die bereits getroffenen Entities.
        // Ist es None, greift der Besitzer gerade nicht an.
        let Some(active_attack) = attacker.active.as_mut() else { continue; };

        // Schaden aus der Waffendefinition: Items -> ItemDef -> WeaponDef -> base.damage
        let damage = items.0.get(&active_attack.item)
            .and_then(|item_def| item_def.weapon.as_ref())
            .map(|weapon_def| weapon_def.base.damage)
            .unwrap_or(0.0);

        // Die vier Eckpunkte der Waffen-Hitbox in Weltkoordinaten.
        // Position, Rotation und Skalierung stecken in der GlobalTransform.
        let weapon_corners = world_corners(weapon_hitbox, weapon_global_tf);

        for (target_entity, mut target_health, target_tf, target_hitbox) in &mut targets {
            // Besitzer überspringen: kein Selbsttreffer
            if target_entity == owner_entity { continue; }
            // Ziel wurde in diesem Angriff schon getroffen: kein Mehrfachschaden
            if active_attack.hit_entities.contains(&target_entity) { continue; }

            // AABB = "Axis-Aligned Bounding Box": ein Rechteck, dessen Kanten
            // parallel zur X- und Y-Achse liegen (nicht gedreht). Gespeichert als
            // untere linke Ecke (min) und obere rechte Ecke (max).
            // Gegner werden nicht rotiert, deshalb reicht eine AABB für sie.
            let target_aabb = target_hitbox.aabb(target_tf.translation.truncate());

            // Überschneidungstest gedrehtes Waffen-Rechteck gegen AABB
            // (Separating Axis Theorem, siehe unten)
            if obb_hits_aabb(&weapon_corners, &target_aabb) {
                active_attack.hit_entities.push(target_entity); // als getroffen merken
                target_health.health_points -= damage;
                println!("HIT {target_entity:?}");
            }
        }
    }
}