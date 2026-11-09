use bevy::prelude::*;
use bevy::sprite::*;
use avian2d::prelude::*;
use std::collections::HashMap;
use crate::ingame::inventory::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::modifier::*;
use crate::components::*;
use crate::constants::*;
use crate::ingame::animation::*;
use crate::ingame::items::structs::*;
use crate::ingame::items::attack::structs::*;

#[derive(Component)]
#[require(Transform, Stats, Health, Mana, Inventory, Equipment, Attacker, Hitbox, Hittable)]
pub struct Player;


//structs für waffenlogik
#[derive(Component)] pub struct PlayerBody;
#[derive(Component)] pub struct HandPivot;
#[derive(Component)] pub struct HeldItemSprite(Option<ItemId>);


impl Player {
    pub fn get_clips(assets: &AssetServer, layouts: &mut Assets<TextureAtlasLayout>) -> AnimClips {
        let mut clips = HashMap::new();
        clips.insert(
            HumanoidState::WalkingLeft.into(),
            AnimClip::new(0, 3, layouts, assets.load("player/character_walking_right.png"), true),
        );
        clips.insert(
            HumanoidState::WalkingRight.into(),
            AnimClip::new(0, 3, layouts, assets.load("player/character_walking_right.png"), true),
        );
        clips.insert(
            HumanoidState::WalkingBack.into(),
            AnimClip::new(0, 3, layouts, assets.load("player/character_walking_back.png"), true),
        );
        clips.insert(
            HumanoidState::WalkingFront.into(),
            AnimClip::new(0, 3, layouts, assets.load("player/character_walking_front.png"), true),
        );
        clips.insert(
            HumanoidState::StandingLeft.into(),
            AnimClip::new(0, 0, layouts, assets.load("player/character_standing_right.png"), true),
        );
        clips.insert(
            HumanoidState::StandingRight.into(),
            AnimClip::new(0, 0, layouts, assets.load("player/character_standing_right.png"), true),
        );
        clips.insert(
            HumanoidState::StandingBack.into(),
            AnimClip::new(0, 0, layouts, assets.load("player/character_standing_back.png"), true),
        );
        clips.insert(
            HumanoidState::StandingFront.into(),
            AnimClip::new(0, 0, layouts, assets.load("player/character_standing_front.png"), true),
        );
        AnimClips { clips }
    }

    pub fn new_entity(
        asset_server: &AssetServer,
        mut texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> impl Bundle { 

        let clips = Player::get_clips(asset_server, texture_atlas_layouts);
        let anim = clips.clips.get(&Anim::Humanoid(HumanoidState::StandingFront)).unwrap();
        let timer = AnimationTimer::new();

        (
            GameEntity,
            Player,
            RigidBody::Kinematic,
            Collider::rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
            WallCollidable,
            Transform::from_xyz(16.0 + PLAYER_WIDTH as f32 / 2.0, 16.0 + PLAYER_HEIGHT as f32 / 2.0, 1.0),
            Velocity { x: 0.0, y: 0.0 },
            Visibility::default(),
            children![
                (
                    PlayerBody,
                    MyEntity,
                    Visibility::default(),
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
                    Anchor::CENTER,
                    Transform::default(),
                    clips,
                    timer,
                ), (
                    HandPivot,
                    Visibility::default(),
                    Transform::default(),
                    children![(
                        HeldItemSprite(None),
                        Hitbox {
                            size: Vec2::new(32.0, 32.0),
                            offset: Vec2::new(0.0, 0.0),
                        },
                        Sprite::default(),
                        Anchor::BOTTOM_LEFT, //weil sprite mittig starten soll
                        Visibility::Hidden,
                    )]
                )
            ],
        )
    }
}