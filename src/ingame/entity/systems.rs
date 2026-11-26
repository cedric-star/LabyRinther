use bevy::prelude::*;
use crate::ingame::entity::structs::*;
use crate::ingame::entity::enemy_waves::*;
use crate::ingame::entity::enemy::*;

pub fn draw_enemy_health_bar(
    enemies: Query<&Health, With<Enemy>>,
    mut bars: Query<(&ChildOf, &mut Sprite, &mut Transform), With<EnemyHealthBar>>,
) {
    const FULL: f32 = 35.0;
    for (child_of, mut sprite, mut tf) in &mut bars {
        let Ok(health) = enemies.get(child_of.parent()) else { continue };
        let ratio = (health.health_points / health.max_health).clamp(0.0, 1.0);
        sprite.custom_size = Some(Vec2::new(FULL * ratio, 2.0));
        // linksbündig halten: Mitte nach links verschieben
        tf.translation.x = -FULL * (1.0 - ratio) * 0.5;
    }
}

pub fn despawn_enemy_by_health(
    enemies: Query<(&Health, Entity), With<Enemy>>,
    mut commands: Commands,
    mut enemy_res: ResMut<Enemies>,
) {
    for (health, entity) in &enemies {
        if health.health_points <= 0.0 {
            commands.entity(entity).despawn();
            if enemy_res.alive != 0 {
                enemy_res.alive = enemy_res.alive.saturating_sub(1);//over/underflow verhinder
            }
        }
    }
}

