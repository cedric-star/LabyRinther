#![allow(non_snake_case)]

use bevy::prelude::*;
use bevy::sprite::*;
use bevy::camera::*;
use bevy::window::*;
use bevy::input_focus::tab_navigation::{TabIndex, TabNavigationPlugin};
use bevy::input_focus::InputDispatchPlugin;
use bevy::picking::hover::Hovered;
use bevy::ui_widgets::*;
use crate::world::world_generator::*;
use crate::world::region::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use std::time::*;

#[derive(Resource)]
pub struct GameSettings {
    pub MOV_SPEED: u64,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            MOV_SPEED: 4,
        }
    }
}

const SETTINGS: &[SettingBinding] = &[
    SettingBinding {
        label: "Move Speed",
        get: |s| s.MOV_SPEED,
        set: |s, v| s.MOV_SPEED = v,
        min: 1,
        max: 30,
    },
    // weitere Settings hier ...
];

#[derive(Component)]
struct GoBackButton;

/// Bindung zwischen Slider-Entity und einem Feld in GameSettings.
#[derive(Component, Clone, Copy)]
struct SettingBinding {
    label: &'static str,
    get: fn(&GameSettings) -> u64,
    set: fn(&mut GameSettings, u64),
    min: u64,
    max: u64,
}

#[derive(Component)]
struct DemoSlider;

#[derive(Component)]
struct DemoSliderThumb;

const SLIDER_TRACK: Color = Color::srgb(0.35, 0.35, 0.35);
const SLIDER_THUMB: Color = Color::srgb(0.9, 0.9, 0.9);

// ---------------------------------------------------------------------------
// Plugin
// ---------------------------------------------------------------------------

pub fn settings_plugin(app: &mut App) {
    // Widget-Plugins nur hinzufügen, falls sie nicht schon woanders aktiv sind.
    if !app.is_plugin_added::<SliderPlugin>() {
        app.add_plugins(UiWidgetsPlugins);
    }
    if !app.is_plugin_added::<InputDispatchPlugin>() {
        app.add_plugins(InputDispatchPlugin);
    }
    if !app.is_plugin_added::<TabNavigationPlugin>() {
        app.add_plugins(TabNavigationPlugin);
    }

    app
        .add_systems(OnEnter(GameState::Settings), settings_setup)
        .add_systems(
            Update,
            (go_back_button, update_slider_pos).run_if(in_state(GameState::Settings)),
        )
        .add_observer(on_setting_change);
}

// ---------------------------------------------------------------------------
// UI-Aufbau
// ---------------------------------------------------------------------------

fn settings_setup(mut commands: Commands, settings: Res<GameSettings>) {
    let root = commands
        .spawn((
            DespawnOnExit(GameState::Settings),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                ..default()
            },
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
        ))
        .id();

    commands.entity(root).with_children(|p| {
        for b in SETTINGS {
            p.spawn(setting_row(*b, &settings));
        }

        p.spawn((
            GoBackButton,
            Button,
            Node {
                width: px(300),
                height: px(65),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            children![(
                Text::new("Go Back"),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],
        ));
    });
}

/// Eine Zeile: Label links, Slider rechts. Das Binding sitzt direkt auf dem Slider.
fn setting_row(b: SettingBinding, settings: &GameSettings) -> impl Bundle {
    (
        Node {
            width: percent(80),
            height: px(65),
            padding: UiRect::horizontal(px(20)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: px(20),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        children![
            (
                Text::new(b.label),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ),
            (
                slider(b.min as f32, b.max as f32, (b.get)(settings) as f32),
                b,
            ),
        ],
    )
}

fn slider(min: f32, max: f32, value: f32) -> impl Bundle {
    (
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
        DemoSlider,
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
                BackgroundColor(SLIDER_TRACK),
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
                    DemoSliderThumb,
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0), // wird von update_slider_pos gesetzt
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}

// ---------------------------------------------------------------------------
// Logik
// ---------------------------------------------------------------------------

/// Wird bei jeder Slider-Änderung aufgerufen: Slider-Wert übernehmen und
/// in GameSettings zurückschreiben.
fn on_setting_change(
    change: On<ValueChange<f32>>,
    mut commands: Commands,
    bindings: Query<&SettingBinding>,
    mut settings: ResMut<GameSettings>,
) {
    // Headless-Slider ändert seinen Wert nicht selbst.
    commands.entity(change.source).insert(SliderValue(change.value));

    if let Ok(b) = bindings.get(change.source) {
        (b.set)(&mut settings, change.value.round() as u64);
    }
}

/// Thumb-Position aus dem Slider-Wert berechnen.
fn update_slider_pos(
    sliders: Query<
        (Entity, &SliderValue, &SliderRange),
        (With<DemoSlider>, Or<(Changed<SliderValue>, Changed<SliderRange>)>),
    >,
    children: Query<&Children>,
    mut thumbs: Query<&mut Node, With<DemoSliderThumb>>,
) {
    for (slider, value, range) in &sliders {
        for child in children.iter_descendants(slider) {
            if let Ok(mut node) = thumbs.get_mut(child) {
                node.left = percent(range.thumb_position(value.0) * 100.0);
            }
        }
    }
}

fn go_back_button(
    query: Query<&Interaction, (Changed<Interaction>, With<GoBackButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            println!("go back to menu");
            next_state.set(GameState::Welcome);
        }
    }
}
