use bevy::prelude::*;
use avian2d::prelude::*;
use crate::ingame::animation::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::modifier::*;
use crate::world::grid::*;
use crate::ingame::entity::enemy::*;

//hat ein enemy einen player gesehen? -> dann soll verfolgung aufgenommen werden!
pub fn set_enemy_player_seen(
    mut enemies: Query<(&mut Enemy, &Transform)>,
    grid: WorldGrid,
    player: Single<&Transform, With<Player>>,
) {
    for (mut enemy, transform) in &mut enemies {
        let can_view = grid.can_view(&enemy, transform, &player);
        if enemy.target != EnemyTarget::NextPlayer && can_view {
            enemy.target = EnemyTarget::NextPlayer;
        }
    }
}

//schaltet zwischen movementstate(warten, laufen, ...)
pub fn set_enemy_movement(
    mut enemies: Query<(&mut Enemy, &CollidingEntities)>,
    time: Res<Time>,
) {
    let delta_t = time.delta_secs();

    for (mut enemy, colliding) in &mut enemies {
        let mut w_state = match &mut enemy.target {
            EnemyTarget::Nothing(s) => { s }
            _ => { continue; }
        };

        let mut next_state: Option<MovementState> = None;

        //wenn eine collision da ist (egal womit) dann richtung ändern und laufen
        if !colliding.is_empty() { 
            next_state = Some(MovementState::Walking(WalkingMovementState::default()));
        }        

        match &mut w_state {
            MovementState::Standing(s) => {
                s.current += delta_t;
                if s.current >= s.max { next_state = Some(MovementState::Walking(WalkingMovementState::default()))}
            }
            MovementState::Walking(w) => {
                w.current += delta_t;
                if w.current >= w.max { next_state = Some(MovementState::Standing(StandingMovementState::default()))}
            }
        }

        if let Some(state) = next_state {
            *w_state = state;
        }

    }
}

//bewege enemies: rumlaufen oder in richtung player
pub fn move_enemies(
    time: Res<Time>,
    mut enemies: Query<(&mut LinearVelocity, &Transform, &mut Anim, &Enemy, &Stats)>,
    player: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    const ACCEL: f32 = 8.0;

    //player position (welt/px)
    let p_pos = player.translation.truncate();

    //für bewegung: beschleunigung
    let t = (ACCEL * time.delta_secs()).min(1.0);

    for (mut l_vel, transform, mut anim, enemy, stats) in &mut enemies {
        //in welche richtung soll der Enemy laufen? -> None = stehen bleiben
        let dir: Option<Vec2> = match &enemy.target {
            EnemyTarget::Nothing(MovementState::Standing(_)) => None,
            EnemyTarget::Nothing(MovementState::Walking(w)) => Some(w.direction),
            EnemyTarget::NextPlayer => {
                let to_player = p_pos - transform.translation.truncate();
                (to_player.length() > enemy.stop_distance).then(|| to_player.normalize())
            }
        };

        //gschwindigkeit und passende animation
        let (desired, new_anim) = match dir {
            Some(d) => (d * stats.get(Stat::WalkSpeed), walking_anim(d)),
            None => (
                Vec2::ZERO,
                match *anim {
                    Anim::Humanoid(s) => s.to_standing().into(),
                    other => other,
                },
            ),
        };

        l_vel.0 = l_vel.0.lerp(desired, t);

        //animation nur setzen, wenn geändert
        if *anim != new_anim {
            *anim = new_anim;
        }
    }
}

/// Lauf-Animation anhand der dominanten Achse der Richtung.
fn walking_anim(dir: Vec2) -> Anim {
    if dir.x.abs() > dir.y.abs() {
        if dir.x > 0.0 { HumanoidState::WalkingRight.into() }
        else { HumanoidState::WalkingLeft.into() }
    } else if dir.y > 0.0 {
        HumanoidState::WalkingBack.into()
    } else {
        HumanoidState::WalkingFront.into()
    }
}