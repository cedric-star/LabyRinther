use bevy::prelude::*;
use bevy::input_focus::tab_navigation::{TabNavigationPlugin};
use bevy::input_focus::InputDispatchPlugin;
use bevy::ui_widgets::{
    SliderPlugin, SliderRange, SliderValue, UiWidgetsPlugins,
    ValueChange,
};
use crate::components::*;
use crate::util::ui::constants::*;
use crate::util::ui::buttons::*;

#[derive(Resource)]
pub struct GameSettings {
    pub mov_speed: u64,
    pub camera_mult: f32,
    pub camera_width: f32,
    pub camera_height: f32,
    pub player_walk_border_width: f32,
    pub player_walk_border_height: f32,
    pub chunks_loading: i32,
    pub ui_scale: f32,
}

impl Default for GameSettings {

    fn default() -> Self {
        #![allow(non_upper_case_globals)]
        const camera_mult: f32 = 2.0;
        const camera_width: f32 = 480.0 * camera_mult;
        const camera_height: f32 = 270.0 * camera_mult;

        Self {
            mov_speed: 4,

            camera_mult,
            camera_width,
            camera_height,

            player_walk_border_width: camera_width / (1.5 * camera_mult),
            player_walk_border_height: camera_height / (1.5 * camera_mult),

            chunks_loading: 6,

            ui_scale: 5.0,
        }
    }
}

macro_rules! setting {
    ($label:expr, $field:ident, $min:expr, $max:expr, $step:expr) => {
        SettingBinding {
            label: $label,
            get: |s| s.$field as f32,
            set: |s, v| s.$field = v as _, // `as _` nimmt automatisch den Typ des Feldes
            min: $min,
            max: $max,
            step: $step,
        }
    };
}

const SETTINGS: &[SettingBinding] = &[
    setting!("Move Speed", mov_speed, 1.0, 12.0, 1.0),
    SettingBinding {
        label: "Camera Zoom",
        get: |s| s.camera_mult,
        set: |s, v| {
            s.camera_mult = v;
            s.camera_width = 480.0 * v;
            s.camera_height = 270.0 * v;
            s.player_walk_border_width = s.camera_width / (1.5 * v);
            s.player_walk_border_height = s.camera_height / (1.5 * v);
        },
        min: 2.0,
        max: 8.0,
        step: 0.1,
    },
    setting!("Chunks Loading", chunks_loading, 2.0, 16.0, 1.0),
    setting!("UI Scale", ui_scale, 2.0, 8.0, 0.1),

    // neues Setting = eine neue Zeile
];

#[derive(Component)]
struct GoBackButton;

#[derive(Component, Clone, Copy)]
struct SettingBinding {
    label: &'static str,
    get: fn(&GameSettings) -> f32,
    set: fn(&mut GameSettings, f32),
    min: f32,
    max: f32,
    step: f32, // 1.0 für ganze Zahlen, 0.1 für Kommazahlen
}

impl SettingBinding {
    fn snap(&self, v: f32) -> f32 {
        ((v / self.step).round() * self.step).clamp(self.min, self.max)
    }

    fn format(&self, v: f32) -> String {
        if self.step >= 1.0 {
            format!("{:.0}", v)
        } else {
            format!("{:.1}", v)
        }
    }
}

#[derive(Component)]
struct DemoSlider;

#[derive(Component)]
struct DemoSliderThumb;

#[derive(Component)]
struct SettingValueText(SettingBinding);

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
            (
                go_back_button,
                update_slider_pos,
                update_value_texts,
                button_visuals,
                slider_thumb_visuals,
            ).run_if(in_state(GameState::Settings)),
        )
        .add_observer(on_setting_change);
}

// ---------------------------------------------------------------------------
// UI-Aufbau
// ---------------------------------------------------------------------------

fn settings_setup(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    previous: Res<LastState>,
    s: Res<GameSettings>,
) {
    let radio_images = ButtonImages::load_btn_radio(&asset_server);
    let prev_text = match previous.0 {
        Some(GameState::Welcome) => { "Go Back" },
        Some(GameState::InGame) => { "Unpause" },
        _ => { "nonono not nog ):" },
    };

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
            BackgroundColor(BG_COL),
        ))
        .id();

    commands.entity(root).with_children(|p| {
        for b in SETTINGS {
            p.spawn(setting_row(*b, &settings, &radio_images));
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
                Text::new(prev_text),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],
        ));
    });
}

/// Eine Zeile: Label links, Slider rechts. Das Binding sitzt direkt auf dem Slider.
fn setting_row(b: SettingBinding, settings: &GameSettings, imgs: &ButtonImages) -> impl Bundle {

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
                slider(b.min, b.max, (b.get)(settings), DemoSlider, DemoSliderThumb, &imgs),
                b,
            ),
            (
                Text::new(b.format((b.get)(settings))),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node { width: px(60), ..default() }, // feste Breite, damit das Layout nicht springt
                SettingValueText(b),
)        ],
    )
}

// ---------------------------------------------------------------------------
// Logik
// ---------------------------------------------------------------------------

fn on_setting_change(
    change: On<ValueChange<f32>>,
    mut commands: Commands,
    bindings: Query<&SettingBinding>,
    mut settings: ResMut<GameSettings>,
) {
    if let Ok(b) = bindings.get(change.source) {
        let value = b.snap(change.value);
        commands.entity(change.source).insert(SliderValue(value));
        (b.set)(&mut settings, value);
    }
}

fn update_value_texts(
    settings: Res<GameSettings>,
    mut texts: Query<(&SettingValueText, &mut Text)>,
) {
    if !settings.is_changed() {
        return;
    }
    for (value, mut text) in &mut texts {
        let b = value.0;
        text.0 = b.format((b.get)(&settings));
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
    previous: Res<LastState>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            println!("go back to {:?}", previous);
            match previous.0 {
                Some(GameState::InGame) => next_state.set(GameState::InGame),
                _ => next_state.set(GameState::Welcome)
            }
        }
    }
}
