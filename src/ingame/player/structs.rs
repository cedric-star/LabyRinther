use bevy::prelude::*;
use bevy::sprite::*;
use crate::ingame::inventory::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::modifier::*;
use crate::components::*;
use crate::constants::*;
use crate::ingame::player::animation::*;


#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[require(Transform, Velocity, Stats, Health, Mana, Inventory, Equipment)]
pub struct Player;


impl Player {
    pub fn new_entity(
        asset_server: &AssetServer,
        mut texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> impl Bundle { 

        let animation_config = AnimationConfig::new(0, 1, 5);
        let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 1, None, None));

        (
            GameEntity,
            Player,
            Sprite {
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                image: asset_server.load("player/standing/player_standing_anim.png"),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_config.first_sprite_index,
                }),
                ..default()
            },
            Transform::from_xyz(16.0, 16.0, 1.0),
            Velocity { x: 0.0, y: 0.0 },
            Anchor::BOTTOM_LEFT,
            animation_config,
        )
    }
}