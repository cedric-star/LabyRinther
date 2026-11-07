use bevy::prelude::*;
use crate::ingame::entity::structs::*;
use crate::ingame::animation::*;
use crate::ingame::player::structs::*;

pub fn move_enemies(
    mut enemies: Query<(&mut Transform, &mut Velocity, &mut Anim, Entity), (With<Enemy>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let enemies_clone: Vec<(Transform, Velocity, Anim, Entity)> = enemies
        .iter() //iter ist mut
        .map(|(t, v, a, e)| (*t, *v, a.clone(), e.clone()))
        .collect();
    //println!("len: {}", enemies_clone.len());

    for (mut transform, mut velocity, mut anim, entity) in &mut enemies {
        let mut other_enemies: Vec<(Transform, Velocity, Anim, Entity)> = enemies_clone
            .clone()
            .iter()
            .filter(|e| e.3 != entity)
            .map(|item| *item)
            .collect();

        //println!("len: {}", other_enemies.len());

        velocity.x = 0.0;
        velocity.y = 0.0;
        let mut new_anim = *anim;

        let p_pos = player.translation;

        if transform.translation.x < player.translation.x {
            velocity.x += 1.0;
            new_anim = HumanoidState::WalkingRight.into();
        }
        if transform.translation.x > player.translation.x {
            velocity.x -= 1.0;
            new_anim = HumanoidState::WalkingLeft.into();
        }
        if transform.translation.y < player.translation.y {
            velocity.y += 1.0;
            new_anim = HumanoidState::WalkingBack.into();
        }
        if transform.translation.y > player.translation.y {
            velocity.y -= 1.0;
            new_anim = HumanoidState::WalkingFront.into();
        }

        if velocity.x == 0.0 && velocity.y == 0.0 {
            if let Anim::Humanoid(s) = new_anim {
                new_anim = s.to_standing().into();
            }
        }
        if *anim != new_anim {
            *anim = new_anim;
        }
        //transform.translation
    }
}