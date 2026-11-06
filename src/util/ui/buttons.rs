use bevy::prelude::*;
use bevy::ui::*;
use bevy::picking::hover::*;
use bevy::ui_widgets::*;
use bevy::ui_widgets::Button;
use std::collections::HashSet;
use bevy::input_focus::tab_navigation::*;
use crate::util::ui::constants::*;

#[derive(Component, Clone)]
pub struct ButtonImages {
    pub normal: Handle<Image>,
    pub hovered: Handle<Image>,
    pub pressed: Handle<Image>,
}

impl ButtonImages {
    pub fn load_btn(asset_server: &AssetServer) -> Self {
        Self {
            normal: asset_server.load("btn_normal.png"),
            hovered: asset_server.load("btn_hovered.png"),
            pressed: asset_server.load("btn_pressed.png"),
        }
    }
    pub fn load_btn_radio(asset_server: &AssetServer) -> Self {
        Self {
            normal: asset_server.load("btn_radio_normal.png"),
            hovered: asset_server.load("btn_radio_hovered.png"),
            pressed: asset_server.load("btn_radio_selected.png"),
        }
    }
}


pub fn button_visuals(
    mut query: Query<
        (
            Entity,
            Ref<Hovered>,
            Option<Ref<Pressed>>,
            &ButtonImages,
            &mut ImageNode,
        ),
        With<Button>,
    >,
    mut removed_pressed: RemovedComponents<Pressed>,
) {
    // Buttons, bei denen Pressed in diesem Frame entfernt wurde
    let just_unpressed: HashSet<Entity> = removed_pressed.read().collect();

    for (entity, hovered, pressed, images, mut image_node) in &mut query {
        let changed = hovered.is_changed()
            || pressed.as_ref().is_some_and(Ref::is_changed)
            || just_unpressed.contains(&entity);

        if !changed {
            continue;
        }

        image_node.image = match (pressed.is_some(), hovered.get()) {
            (true, _) => images.pressed.clone(),
            (false, true) => images.hovered.clone(),
            (false, false) => images.normal.clone(),
        };
    }
}

pub fn slider_thumb_visuals(
    sliders: Query<(Entity, &Hovered, &SliderDragState), With<Slider>>,
    children: Query<&Children>,
    mut thumbs: Query<(&ButtonImages, &mut ImageNode), With<SliderThumb>>,
) {
    for (slider, hovered, drag) in &sliders {
        for child in children.iter_descendants(slider) {
            if let Ok((images, mut node)) = thumbs.get_mut(child) {
                node.image = if drag.dragging {
                    images.pressed.clone()
                } else if hovered.get() {
                    images.hovered.clone()
                } else {
                    images.normal.clone()
                };
            }
        }
    }
}

pub fn my_button(
    marker: impl Bundle,
    label: &str,
    height: f32,
    images: &ButtonImages,
) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        marker,
        Node {
            width: px(300),
            height: px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(px(8)),
            ..default()
        },
        ImageNode::new(images.normal.clone()),
        images.clone(),
        children![(
            Text::new(label.to_string()),
            TextFont { font_size: FontSize::Px(33.0), ..default() },
            TextColor(TXT_COL),
        )],
    )
}

pub fn slider(
    min: f32,
    max: f32,
    value: f32,
    slider_marker: impl Bundle,
    thumb_marker: impl Bundle,
    images: &ButtonImages
) -> impl Bundle {
    (
        slider_marker,
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: percent(30),
            ..default()
        },
        Name::new("Slider"),
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
            ..Default::default()
        },
        SliderValue(value),
        SliderRange::new(min, max),
        TabIndex(0),
        Children::spawn((
            // Slider background rail
            Spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(TXT_COL),
            )),
            // Unsichtbarer Track, damit der Thumb per Prozent positioniert werden kann.
            // 12px kürzer, damit der Thumb Platz hat.
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    thumb_marker,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(40),
                        height: px(40),
                        position_type: PositionType::Absolute,
                        left: px(0),
                        top: px((SLIDER_H - THUMB) / 2.0),
                        margin: UiRect::left(px(-THUMB / 2.0)),
                        ..default()
                    },
                    ImageNode::new(images.normal.clone()),
                    images.clone(),
                )],
            )),
        )),
    )
}
