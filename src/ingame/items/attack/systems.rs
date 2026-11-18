use bevy::prelude::*;
use avian2d::prelude::*;
use crate::ingame::items::structs::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::attack::structs::*;
use crate::ingame::entity::structs::*;

//setzt im active attack cooldown/hit zeiten
//setzt active attack auf none wenn zeit vorbei
pub fn set_attacker(
    time: Res<Time>,
    items: Res<Items>,
    mut attackers: Query<&mut Attacker>,
) {
    let dt = time.delta_secs(); //zeit zwischen letztem frame und aktuellem

    //iteriere durch alle attacekr
    for mut attacker in &mut attackers {

        //vergangene zeit von cooldown subtrahieren
        attacker.cooldown_left = (attacker.cooldown_left - dt).max(0.0);

        //active struct mut holen und weapon item holen
        let Some(active) = attacker.active.as_mut() else { continue; };
        let Some(weapon) = items.0
            .get(&active.item)
            .and_then(|d| d.weapon.as_ref()) 
        else {
            attacker.active = None; //kein active attack, wenn waffe nicht gefunden
            continue;
        };

        //vergangene attackzeit setzen
        active.elapsed += dt;

        let is_over = match &weapon.kind {
            WeaponKind::Dagger(def) => {
                if active.elapsed >= def.hit_at {
                    active.hit_fired = true; //auf active attack hit setzen, wenn zeit gekommen ist
                }
                active.elapsed >= def.duration + def.cooldown
            }
        };

        if is_over { attacker.active = None; }

    }
}

//collider für waffen ein und ausschalten
fn set_hit_enabled(
    commands: &mut Commands,
    entity: Entity,
    enabled: bool,
    is_disabled: bool,
) {
    if enabled && is_disabled {
        commands.entity(entity).remove::<ColliderDisabled>();
    } else if !enabled && !is_disabled {
        commands.entity(entity).insert(ColliderDisabled);
    }
}

//animiert item in playerhand
//iteriert über attacker und setzt zeitbasierte dinge
    // -> animationszeiten
    // -> ob collider für waffe an ist
    // -> ob sprite sichtbar/unsichtbar ist
pub fn animate_held_item(
    mut commands: Commands,
    items: Res<Items>,
    asset_server: Res<AssetServer>,
    attackers: Query<&Attacker>,
    pivots: Query<&ChildOf, With<HandPivot>>,
    mut sprites: Query<(
        Entity,
        &ChildOf,
        &mut Transform,
        &mut Visibility,
        &mut Sprite,
        Has<ColliderDisabled> //bool, ob colliderdisabled existiert in entity
    ), With<HeldItemSprite>>,
) {
    for (entity, parent, mut tf, mut vis, mut sprite, is_disabled) in &mut sprites {
        let Ok(pivot_parent) = pivots.get(parent.parent()) else { continue; };
        let Ok(attacker) = attackers.get(pivot_parent.parent()) else {continue; };

        let mut visible = false;    //ob waffe sichtbar ist
        let mut hit_window = false; //ob collider aktiv ist

        //wenn aktier attack am laufen ist
        if let Some(active) = &attacker.active {
            //wenn item gefunden wird (bzw. waffe)
            if let Some(item) = items.0.get(&active.item) {
                //weapon struct/informationen aus item herausziehen
                if let Some(weapon) = item.weapon.as_ref() {
                    //angriff je nach waffenart animieren
                    match &weapon.kind {
                        WeaponKind::Dagger(def) => {
                            //wenn vergangene zeit innerhalb der animationszeit liegt
                            if active.elapsed < def.duration {
                                visible = true; //wenn zeit innerhalb animation -> sprite zeichnen
                                hit_window = active.elapsed >= def.hit_at; //collider einschalten, sobald hit_at punkt erreicht ist

                                //bild einmalig setzen, nur wenn sich es ändert, neu setzen
                                let handle = asset_server.load(item.asset.clone());
                                if sprite.image != handle { sprite.image = handle; }

                            //linearvergehende zeit auf sinuskruve für bewegung mappen
                                //animationszeit zwischen 0 und 1 normalisieren (quasi prozente)
                                let t = (active.elapsed / def.duration).clamp(0.0, 1.0); 
                                //t = 0-1 -> 0/1 pi = 0, alles dazwischen wird zu einer sinuskurve (sinus rechnet mit pi werten)
                                let thrust = (t * std::f32::consts::PI).sin();
                                //zielrichtung der waffe festlegen
                                let dir = active.aim.normalize_or_zero();

                                //richtungsvektor * länge = vektor mit korekter richtung und könge
                                // * thrust verschiebt vektorurpsrung auf sinuskurve
                                //extend macht aus vec2 einen vec3 mit z=0
                                tf.translation = (dir * def.distance * thrust).extend(0.0);

                                tf.rotation = Quat::from_rotation_z(
                                    dir.to_angle() - 45.0_f32.to_radians(),
                                );
                            }
                        }
                    }
                }
            }
        }

        //sprite zurücksetzen wenn unsichtbar
        if !visible {
            tf.translation = Vec3::ZERO;
            tf.rotation = Quat::IDENTITY;
        }

        //vis auf passenden enum setzen -> nur wenn es sich ändert
        vis.set_if_neq(if visible { Visibility::Visible } else  { Visibility::Hidden });

        //schlussendlich ColliderDisabled setzen oder löschen, um waffencollider an/auszuschalten
        set_hit_enabled(&mut commands, entity, hit_window, is_disabled);

    }
}

//erkennt überlappung zwischen waffen-sensor und setzt damage
pub fn weapon_contact(
    items: Res<Items>,
    weapons: Query<(&CollidingEntities, &ChildOf), (With<HeldItemSprite>, Without<ColliderDisabled>)>,
    pivots: Query<&ChildOf, With<HandPivot>>,
    mut attackers: Query<(&mut Attacker, &Transform)>,
    mut targets: Query<(&mut Health, &Transform, &ComputedMass, Forces), With<Hittable>>,
) {
    for (colliding, parent) in &weapons {
        let Ok(pivot_parent) = pivots.get(parent.parent()) else { continue; };
        let owner = pivot_parent.parent();
        let Ok((mut attacker, attacker_tf)) = attackers.get_mut(owner) else { continue; };
        let Some(active) = attacker.active.as_mut() else { continue; };

        let weapon_def: WeaponDef = match items.0.get(&active.item) {
            Some(item) => {
                match &item.weapon {
                    Some(weapon) => { *weapon }
                    None => { continue; }
                }
            }
            None => { continue; }
        };

        let damage = weapon_def.base.damage;

        for &target in colliding.iter() {
            if target == owner || active.hit_entities.contains(&target) { continue; }

            if let Ok((mut health, transform, mass, mut forces)) = targets.get_mut(target) {//hier passiert der hit
                health.health_points -= damage;
                active.hit_entities.push(target);

                let knockback = weapon_def.base.knockback;
                let impulse_vec = active.aim.normalize_or_zero() * knockback * mass.value();
                forces.apply_linear_impulse(impulse_vec);
            }
        }
    }
}