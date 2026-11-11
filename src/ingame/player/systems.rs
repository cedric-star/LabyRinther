use bevy::prelude::*;
use bevy::window::*;
use avian2d::prelude::*;
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
use crate::ingame::animation::*;
use crate::ingame::items::attack::structs::*;

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
    player: Single<(&mut LinearVelocity, &Transform, &Stats), (With<Player>, Without<Camera2d>)>,
    mut cam: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    s: Res<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_anim: Single<&mut Anim, With<PlayerBody>>,
) {
    //player variablen/dinge
    let (mut p_vel, p_transform, p_stats) = player.into_inner();
    
    let mov_speed = if keys.pressed(KeyCode::ShiftLeft) {
        p_stats.get(Stat::SprintSpeed)
    } else { p_stats.get(Stat::WalkSpeed) };
     

    //keys auslesen --> nur richtung setzen
    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }
    let dir = dir.normalize_or_zero();

    //bewegung linearvel setzen
    p_vel.0 = dir * mov_speed;

    //animation
    let mut new_anim: Anim = **player_anim;
    if dir != Vec2::ZERO {
        new_anim = if dir.x.abs() > dir.y.abs() {
            if dir.x > 0.0 { HumanoidState::WalkingRight.into() }
            else { HumanoidState::WalkingLeft.into() }
        } else if dir.y > 0.0 {
            HumanoidState::WalkingBack.into()
        } else {
            HumanoidState::WalkingFront.into()
        };
    } else if let Anim::Humanoid(st) = new_anim {
        new_anim = st.to_standing().into();
    }
    if **player_anim != new_anim {
        **player_anim = new_anim;
    }

    //kamera anhand von player pos mitbewegen
    let p = p_transform.translation.truncate();
    let offset = p - cam.translation.truncate();
    let clamped = offset.clamp(
        Vec2::new(-s.player_walk_border_width, -s.player_walk_border_height),
        Vec2::new(s.player_walk_border_width, s.player_walk_border_height),
    );
    let target = p - clamped;
    cam.translation.x = target.x;
    cam.translation.y = target.y;

    //settings menü öffnen
    if keys.pressed(KeyCode::Escape) {
        next_state.set(GameState::Settings);
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
    
    if mouse.just_pressed(MouseButton::Left) {  }
    else if mouse.pressed(MouseButton::Left) {  }
    else { return; }

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

    let origin = transform.translation.truncate(); 
    let aim = (world_pos - origin).normalize_or_zero();

    //attack erstellen und in player component setzen
    attacker.active = Some(ActiveAttack {
        item: item.id,
        elapsed: 0.0,
        hit_fired: false,
        aim,
        hit_entities: Vec::new(),
    });


}