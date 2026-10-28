use bevy::prelude::*;
use std::collections::HashMap;
use crate::ingame::entity::structs::*;

#[derive(Component, Debug, Clone)]
pub struct AnimClip {
    pub first: usize,
    pub last: usize,
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
    pub repeat: bool,
}

#[derive(Clone, Copy, Component, Debug, Hash, Eq, PartialEq)]
pub enum Anim {
    Humanoid(HumanoidState),
}

#[derive(Clone, Debug, Component)]
pub struct AnimClips {
    pub clips: HashMap<Anim, AnimClip>,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);


impl AnimClip {
    pub fn new(first: usize, last: usize, layouts: &mut Assets<TextureAtlasLayout>, image: Handle<Image>, repeat: bool) -> Self {
        let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 1, None, None));
        Self {
            first,
            last,
            layout,
            image,
            repeat,
        }
    }
}
impl AnimationTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(0.15, TimerMode::Repeating))
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HumanoidState {
    WalkingLeft,
    WalkingRight,
    WalkingBack,
    WalkingFront,
    StandingLeft,
    StandingRight,
    StandingBack,
    StandingFront,
}

impl From<HumanoidState> for Anim {
    fn from(s: HumanoidState) -> Self { Anim::Humanoid(s)}
}



/// Bei State-Wechsel Bild/Atlas umstellen
pub fn switch_clip(
    mut q: Query<(&Anim, &AnimClips, &mut AnimationTimer, &mut Sprite), Changed<Anim>>,
) {
    for (anim, clips, mut timer, mut sprite) in &mut q {
        let Some(clip) = clips.clips.get(anim) else { continue };
        sprite.image = clip.image.clone();
        sprite.texture_atlas = Some(TextureAtlas {
            layout: clip.layout.clone(),
            index: clip.first,
        });
        sprite.flip_x = matches!(
            anim,
            Anim::Humanoid(HumanoidState::WalkingLeft | HumanoidState::StandingLeft)
        );
        timer.0.reset();
    }
}

/// Frames weiterschalten
pub fn animate_entity(
    time: Res<Time>,
    mut q: Query<(&Anim, &AnimClips, &mut AnimationTimer, &mut Sprite)>,
) {
    for (anim, clips, mut timer, mut sprite) in &mut q {
        let Some(clip) = clips.clips.get(anim) else { continue };
        if timer.0.tick(time.delta()).just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= clip.last {
                    if clip.repeat { clip.first } else { clip.last }
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}