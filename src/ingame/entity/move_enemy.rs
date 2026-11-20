use bevy::prelude::*;
use avian2d::prelude::*;
use crate::ingame::entity::structs::*;
use crate::ingame::animation::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::modifier::*;
use crate::components::*;

//hat ein enemy einen player gesehen? -> dann soll verfolgung aufgenommen werden!
pub fn set_enemy_player_seen(
    mut enemies: Query<(&mut Enemy, &Transform)>,
    chunks: Query<&Chunk>,
    player: Single<&Transform, With<Player>>,
) {
    let player_tile = player
                    .translation
                    .truncate()
                    .as_ivec2()
                    .world_to_tile();

    
                
    let chunks: Vec<&Chunk> = chunks.iter().collect();
    
    for (enemy, transform) in &mut enemies {
        let enemy_tile = transform
            .translation
            .truncate()
            .as_ivec2()
            .world_to_tile();
    }
}

//bewege eneies in richtung player
pub fn move_enemies(
    time: Res<Time>,
    mut enemies: Query<(&mut LinearVelocity, &Transform, &mut Anim, &Enemy, &Stats)>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    const ACCEL: f32 = 8.0;

    let p_pos = player.translation.truncate();
    let t = (ACCEL * time.delta_secs()).min(1.0);



    for (mut l_vel, transform, mut anim, enemy, stats) in &mut enemies {
        let to_player = p_pos - transform.translation.truncate(); //wohin soll enemy laufen?
        let mut new_anim = *anim;

        //wenn enemy vom gegner weg ist (länge des vectors)
        if to_player.length() > enemy.stop_distance {
            let direction = to_player.normalize(); //richtungsvektor länge 1
            let desired = direction * stats.get(Stat::WalkSpeed); //vektor auf richtung * geschwindigkeit setzen

            l_vel.0 = l_vel.0.lerp(desired, t);

            //animation je nach richtung setzen
            new_anim = if direction.x.abs() > direction.y.abs() {
                if direction.x > 0.0 { HumanoidState::WalkingRight.into() }
                else { HumanoidState::WalkingLeft.into() }
            } else if direction.y > 0.0 { HumanoidState::WalkingBack.into() }
            else { HumanoidState::WalkingFront.into() }
        } else {
            l_vel.0 = l_vel.0.lerp(Vec2::ZERO, t);
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