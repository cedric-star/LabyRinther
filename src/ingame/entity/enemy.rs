use bevy::prelude::*;
use avian2d::prelude::*;
use crate::ingame::entity::structs::*;
use crate::ingame::animation::*;
use crate::ingame::player::structs::*;

pub fn move_enemies(
    mut enemies: Query<(&mut LinearVelocity, &Transform, &mut Anim, &Enemy)>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let p_pos = player.translation.truncate();

    for (mut l_vel, transform, mut anim, enemy) in &mut enemies {
        let to_player = p_pos - transform.translation.truncate(); //wohin soll enemy laufen?
        let mut new_anim = *anim;

        //wenn enemy vom gegner weg ist (länge des vectors)
        if to_player.length() > enemy.stop_distance {
            let direction = to_player.normalize(); //richtungsvektor länge 1
            l_vel.0 = direction * enemy.speed; //vektor auf richtung * geschwindigkeit setzen

            //animation je nach richtung setzen
            new_anim = if direction.x.abs() > direction.y.abs() {
                if direction.x > 0.0 { HumanoidState::WalkingRight.into() }
                else { HumanoidState::WalkingLeft.into() }
            } else if direction.y > 0.0 { HumanoidState::WalkingBack.into() }
            else { HumanoidState::WalkingFront.into() }
        } else {
            l_vel.0 = Vec2::ZERO;
            if let Anim::Humanoid(s) = new_anim {
                new_anim = s.to_standing().into();
            }
        }

        //animation nur setzen, wenn geändert
        if *anim != new_anim {
            *anim = new_anim;
        }
        
    }
}