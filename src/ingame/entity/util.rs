use bevy::prelude::*;
use bevy::sprite::*;
use avian2d::prelude::*;
use crate::ingame::entity::structs::*;
use crate::components::*;
use crate::ingame::player::structs::*;
use crate::ingame::animation::*;
use crate::constants::*;

impl Default for Health {
    fn default() -> Self {
        Self {
            health_points: 100.0,
            max_health: 100.0,
            health_regen: 1.0,      
        }
    }
}

impl Default for Mana {
    fn default() -> Self {
        Self {
            mana_points: 0.0,
            max_mana: 100.0,
            mana_regen: 1.0,    
        }
    }
}



pub fn get_enemy_bundle(
    pos: Cords,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,

) -> impl Bundle {

    let clips = Player::get_clips(asset_server, texture_atlas_layouts);
    let anim = clips.clips.get(&Anim::Humanoid(HumanoidState::StandingFront)).unwrap();
    let timer = AnimationTimer::new();
    
    (
        RigidBody::Dynamic,
        Collider::circle(PLAYER_WIDTH / 2.0),
        LockedAxes::ROTATION_LOCKED,//damit sich gegner nicht hässlich dreht
        GameEntity,
        Enemy::new(),
        Transform::from_xyz(pos.x as f32, pos.y as f32, 1.0),
        Visibility::Visible,
        WallCollidable,
        Anchor::CENTER,
        MyEntity,
        Anim::Humanoid(HumanoidState::StandingFront),
        Sprite {
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            image: anim.image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: anim.layout.clone(),
                index: anim.first.clone(),
            }),
            ..default()
        },
        clips,
        timer,
        children![
            (
                Sprite { custom_size: Some(Vec2::new(35.0, 2.0)), color: Color::srgb(1.0, 0.0, 0.0), ..default() },
                Transform::from_xyz(0.0, (PLAYER_HEIGHT / 2.0) + 2.0, 0.1),
                EnemyHealthBar,
            ),
            (
                Sprite { custom_size: Some(Vec2::new(35.0, 2.0)), color: Color::srgb(0.5, 0.1, 0.1), ..default() },
                Transform::from_xyz(0.0, (PLAYER_HEIGHT / 2.0) + 2.0, 0.05),
                EnemyHealthBarBackground,
            ),
        ],
    )
}
