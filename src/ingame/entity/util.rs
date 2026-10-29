use bevy::prelude::*;
use bevy::sprite::*;
use crate::ingame::entity::structs::*;
use crate::util::ui::constants::*;
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
    mut texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,

) -> impl Bundle {

    let clips = Player::get_clips(asset_server, texture_atlas_layouts);
    let anim = clips.clips.get(&Anim::Humanoid(HumanoidState::StandingFront)).unwrap();
    let timer = AnimationTimer::new();
    
    (
        GameEntity,
        Enemy::new(),
        Transform::from_xyz(pos.x as f32, pos.y as f32, 1.0),
        Velocity { x: 0.0, y: 0.0 },
        Visibility::Visible,
        WallCollidable,
        children![
            (
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
                Anchor::BOTTOM_LEFT,
                Transform::default(),
                clips,
                timer,
            )
        ],
    )
}