use bevy::prelude::*;
use bevy::window::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::settings::settings_screen::*;
use crate::ingame::player::util::*;
use crate::ingame::player::structs::*;
use crate::ingame::items::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::modifier::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::items::util::*;
use crate::ingame::items::attack::*;
use crate::ingame::animation::*;


pub fn mana_regen(mut q: Query<(&mut Mana, &Stats), With<Player>>) {
    for (mut mana, stats) in &mut q {
        if mana.mana_points < mana.max_mana {
            mana.mana_points = (mana.mana_points + stats.get(Stat::ManaRegen)).min(mana.max_mana);
        }
    }
}

pub fn health_regen(mut q: Query<(&mut Health, &Stats), With<Player>>) {
    for (mut hp, stats) in &mut q {
        if hp.health_points < hp.max_health {
            hp.health_points = (hp.health_points + stats.get(Stat::HealthRegen)).min(hp.max_health);
        }
    }
}

//wenn inventar unsichtbar und item das man benutzen kann ausgerüstet ist, dann benutzen
pub fn use_item_system(
    mouse: Res<ButtonInput<MouseButton>>,
    show_inventory: Single<&InventoryShow, With<InventoryRoot>>,
    items: Res<Items>,
    mut q: Query<(&mut Inventory, &mut Stats), With<Player>>,
) {
    if !mouse.just_pressed(MouseButton::Right) { return; }
    if show_inventory.0 == true { return; }

    let Ok((mut inv, mut stats)) = q.single_mut() else { return };
    use_selected_item(&items, &mut inv, &mut stats);
}

pub fn controls(
    keys: Res<ButtonInput<KeyCode>>,
    transform: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut cam_vel: Single<&mut Velocity, (With<Camera2d>, Without<Player>)>,
    cam_transform: Single<&Transform, (With<Camera2d>, Without<Player>)>,
    s: Res<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player: Single<(&mut Velocity, &Stats), (With<Player>, Without<Camera2d>)>,
    mut player_anim: Single<&mut Anim, With<PlayerBody>>,
) {
    let (velocity, stats) = &mut *player;
    let mov_speed = s.mov_speed as f32 * stats.get(Stat::MoveSpeed);
    velocity.x = 0.0;
    velocity.y = 0.0;
    cam_vel.x = 0.0;
    cam_vel.y = 0.0;

    let mut new_anim: Anim = **player_anim;
    let cam_pos = cam_transform.translation;

    if keys.pressed(KeyCode::KeyW) {
        velocity.y = mov_speed;
        if transform.translation.y + PLAYER_HEIGHT >= cam_pos.y + s.player_walk_border_height {
            cam_vel.y = mov_speed;
        } else {
            new_anim = HumanoidState::WalkingBack.into();
        }
    }
    if keys.pressed(KeyCode::KeyS) {
        velocity.y = -mov_speed;
        if transform.translation.y <= cam_pos.y - s.player_walk_border_height {
            cam_vel.y = -mov_speed;
        } else {
            new_anim = HumanoidState::WalkingFront.into();
        }
        
    }
    if keys.pressed(KeyCode::KeyA) {
        velocity.x = -mov_speed;
        if transform.translation.x <= cam_pos.x - s.player_walk_border_width {
            cam_vel.x = -mov_speed;
        } else {
            new_anim = HumanoidState::WalkingLeft.into();
        }
    }
    if keys.pressed(KeyCode::KeyD) {
        velocity.x = mov_speed;
        if transform.translation.x + PLAYER_WIDTH >= cam_pos.x + s.player_walk_border_width {
            cam_vel.x = mov_speed;
        } else {
            new_anim = HumanoidState::WalkingRight.into();
        }
    }

    // Keine Bewegung -> in Blickrichtung stehen bleiben
    if velocity.x == 0.0 && velocity.y == 0.0 {
        if let Anim::Humanoid(s) = new_anim {
            new_anim = standing_of(s).into();
        }
    }

    if **player_anim != new_anim {
        **player_anim = new_anim;
    }

    if keys.pressed(KeyCode::Escape) {
        next_state.set(GameState::Settings);
    }
}

fn standing_of(s: HumanoidState) -> HumanoidState {
    use HumanoidState::*;
    match s {
        WalkingLeft | StandingLeft => StandingLeft,
        WalkingRight | StandingRight => StandingRight,
        WalkingBack | StandingBack => StandingBack,
        WalkingFront | StandingFront => StandingFront,
    }
}

pub fn movement(
    mut query_player: Query<(&mut Transform, &Velocity), With<WallCollidable>>,
    chunk_pool: Res<ChunkPool>,
    chunks: Query<&Chunk>,
) {
    for (mut transform, velocity) in &mut query_player {
        let mut pos = transform.translation;

        // X-Achse
        let nx = pos.x + velocity.x;
        if collides(nx, pos.y, &chunk_pool, &chunks) {
            if velocity.x > 0.0 {
                pos.x = ((nx + PLAYER_WIDTH) / TILE_SIZE).floor() * TILE_SIZE - PLAYER_WIDTH;
            } else if velocity.x < 0.0 {
                pos.x = ((nx / TILE_SIZE).floor() + 1.0) * TILE_SIZE;
            }
        } else {
            pos.x = nx;
        }

        // Y-Achse
        let ny = pos.y + velocity.y;
        if collides(pos.x, ny, &chunk_pool, &chunks) {
            if velocity.y > 0.0 {
                pos.y = ((ny + PLAYER_HEIGHT) / TILE_SIZE).floor() * TILE_SIZE - PLAYER_HEIGHT;
            } else if velocity.y < 0.0 {
                pos.y = ((ny / TILE_SIZE).floor() + 1.0) * TILE_SIZE;
            }
        } else {
            pos.y = ny;
        }

        transform.translation = pos;
    }
}

pub fn weapon_use(
    mouse: Res<ButtonInput<MouseButton>>,
    items: Res<Items>,
    player_data: Single<(&Inventory, &Transform, &mut Attacker), With<Player>>,
    show_inventory: Single<&InventoryShow, With<InventoryRoot>>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (inventory, transform, mut attacker) = player_data.into_inner();
    let pos = transform.translation;
    
    if !mouse.just_pressed(MouseButton::Left) { return; }

    //hier ändern, wenn meherere angriffre gleicherzeitig
    //dann angriffe im Attacker als vec<opt<...>> speichern
    if attacker.active.is_some() || attacker.cooldown_left > 0.0 { return; }

    if show_inventory.0 == true { return; }
    let Some(item) = inventory.hotbar[inventory.selected].stack else { return; };
    let item = items.0
        .get(&item.id)
        .unwrap();
    if !item.categories.contains(&Category::Weapon) { return; } 

    // Zielrichtung: Spielermitte -> Mausposition in Weltkoordinaten
    let Some(cursor) = windows.single().ok().and_then(|w| w.cursor_position()) else { return; };
    let (cam, cam_tf) = *camera;
    let Ok(world_pos) = cam.viewport_to_world_2d(cam_tf, cursor) else { return; };

    let origin = transform.translation.truncate()
        + Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT) * 0.5; // wegen Anchor::BOTTOM_LEFT
    let aim = (world_pos - origin).normalize_or_zero();

    //attack erstellen und in player component setzen
    attacker.active = Some(ActiveAttack {
        item: item.id,
        elapsed: 0.0,
        hit_fired: false,
        aim,
    });


}