use bevy::picking::hover::*;
use bevy::prelude::*;
use bevy::ui::*;
use bevy::ui_widgets::*;

use crate::components::*;
use crate::settings::aspect_ratio::*;
use crate::settings::settings::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;

// ---------------------------------------------------------------------------
// Slider-Bindings (Zahlen-Settings)
// ---------------------------------------------------------------------------

/// Bindet ein Feld aus `GameSettings.ui` an einen Slider.
macro_rules! ui_setting {
    ($label:expr, $field:ident, $min:expr, $max:expr, $step:expr) => {
        SettingBinding {
            label: $label,
            get: |s| s.ui.$field as f32,
            set: |s, v| s.ui.$field = v as _, // `as _` nimmt automatisch den Typ des Feldes
            min: $min,
            max: $max,
            step: $step,
        }
    };
}

/// Neues Slider-Setting = eine neue Zeile. Abgeleitete Werte werden
/// automatisch über `GameSettings::recompute()` nachgezogen.
const SETTINGS: &[SettingBinding] = &[
    ui_setting!("Camera Zoom", camera_mult, 2.0, 8.0, 0.1),
    ui_setting!("Chunks Loading", chunks_loading, 2.0, 16.0, 1.0),
    ui_setting!("UI Scale", ui_scale, 2.0, 8.0, 0.1),
    // mov_speed & Co. sind bewusst NICHT hier -> nicht im UI änderbar
];

// ---------------------------------------------------------------------------
// Auswahl-Settings (Buttons)
// ---------------------------------------------------------------------------

const ASPECTS: [(AspectRatio, &str); 4] = [
    (AspectRatio::R16_9, "16:9"),
    (AspectRatio::R16_10, "16:10"),
    (AspectRatio::R4_3, "4:3"),
    (AspectRatio::R21_9, "21:9"),
];

const COL_BG: Color = Color::srgb(0.15, 0.15, 0.15);
const COL_SELECTED: Color = Color::srgb(0.30, 0.45, 0.30);
const COL_TEXT: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub struct AspectButton(AspectRatio);

#[derive(Component)]
pub struct ResolutionButton(f32); // Fensterhöhe

// ---------------------------------------------------------------------------
// Komponenten
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct GoBackButton;

#[derive(Component, Clone, Copy)]
pub struct SettingBinding {
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
pub struct DemoSlider;

#[derive(Component)]
pub struct DemoSliderThumb;

#[derive(Component)]
pub struct SettingValueText(SettingBinding);

// ---------------------------------------------------------------------------
// UI-Aufbau
// ---------------------------------------------------------------------------

pub fn settings_setup(
    mut commands: Commands,
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    previous: Res<LastState>,
) {
    let radio_images = ButtonImages::load_btn_radio(&asset_server);
    let prev_text = match previous.0 {
        Some(GameState::Welcome) => "Go Back",
        Some(GameState::InGame) => "Unpause",
        _ => "nonono not nog ):",
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
        // Slider-Zeilen
        for b in SETTINGS {
            p.spawn(setting_row(*b, &settings, &radio_images));
        }

        // Seitenverhältnis
        spawn_option_row(
            p,
            "Aspect Ratio",
            ASPECTS.iter().map(|(a, l)| (l.to_string(), AspectButton(*a))),
        );

        // Auflösung (Fensterhöhe, Breite folgt aus dem Seitenverhältnis)
        spawn_option_row(
            p,
            "Resolution",
            WINDOW_HEIGHTS
                .iter()
                .map(|h| (format!("{}p", *h as u32), ResolutionButton(*h))),
        );

        // Zurück / Unpause
        p.spawn((
            GoBackButton,
            Button,
            Hovered::default(),
            Node {
                width: px(300),
                height: px(65),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(COL_BG),
            children![(
                Text::new(prev_text),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(COL_TEXT),
            )],
        ));
    });
}

/// Eine Zeile: Label links, Slider mitte, Wert rechts. Das Binding sitzt direkt auf dem Slider.
pub fn setting_row(b: SettingBinding, settings: &GameSettings, imgs: &ButtonImages) -> impl Bundle {
    (
        row_node(),
        BackgroundColor(COL_BG),
        children![
            (
                Text::new(b.label),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(COL_TEXT),
            ),
            (
                slider(b.min, b.max, (b.get)(settings), DemoSlider, DemoSliderThumb, &imgs),
                b,
            ),
            (
                Text::new(b.format((b.get)(settings))),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(COL_TEXT),
                Node { width: px(60), ..default() }, // feste Breite, damit das Layout nicht springt
                SettingValueText(b),
            ),
        ],
    )
}

fn row_node() -> Node {
    Node {
        width: percent(80),
        height: px(65),
        padding: UiRect::horizontal(px(20)),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        column_gap: px(20),
        ..default()
    }
}

/// Zeile mit Label links und einer Reihe von Buttons rechts.
/// `C` ist die Marker-Komponente des jeweiligen Buttons (AspectButton, ResolutionButton, ...).
fn spawn_option_row<C: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    options: impl IntoIterator<Item = (String, C)>,
) {
    parent
        .spawn((row_node(), BackgroundColor(COL_BG)))
        .with_children(|row| {
            row.spawn((
                Text::new(label),
                TextFont { font_size: FontSize::Px(33.0), ..default() },
                TextColor(COL_TEXT),
            ));

            row.spawn(Node {
                column_gap: px(10),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|buttons| {
                for (text, marker) in options {
                    buttons.spawn((
                        marker,
                        Button,
                        Hovered::default(),
                        Node {
                            min_width: px(90),
                            height: px(45),
                            padding: UiRect::horizontal(px(12)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(COL_BG),
                        children![(
                            Text::new(text),
                            TextFont { font_size: FontSize::Px(26.0), ..default() },
                            TextColor(COL_TEXT),
                        )],
                    ));
                }
            });
        });
}

// ---------------------------------------------------------------------------
// Logik
// ---------------------------------------------------------------------------

pub fn on_setting_change(
    change: On<ValueChange<f32>>,
    mut commands: Commands,
    bindings: Query<&SettingBinding>,
    mut settings: ResMut<GameSettings>,
) {
    if let Ok(b) = bindings.get(change.source) {
        let value = b.snap(change.value);
        commands.entity(change.source).insert(SliderValue(value));
        (b.set)(&mut settings, value);
        settings.recompute(); // abgeleitete Werte (Kamera, Walk-Border, ...) nachziehen
    }
}

/// `Pressed` ist solange vorhanden, wie der Button gehalten wird. Deshalb wird nur
/// geschrieben, wenn sich der Wert wirklich ändert (sonst würde `GameSettings`
/// jeden Frame als geändert markiert und Kamera/Fenster ständig neu gesetzt).
pub fn aspect_button_system(
    query: Query<(Has<Pressed>, &AspectButton)>,
    mut settings: ResMut<GameSettings>,
) {
    for (pressed, btn) in &query {
        if pressed && settings.ui.aspect != btn.0 {
            settings.ui.aspect = btn.0;
            settings.recompute();
        }
    }
}

pub fn resolution_button_system(
    query: Query<(Has<Pressed>, &ResolutionButton)>,
    mut settings: ResMut<GameSettings>,
) {
    for (pressed, btn) in &query {
        if pressed && settings.ui.window_height != btn.0 {
            settings.ui.window_height = btn.0;
            settings.recompute();
        }
    }
}

/// Markiert den aktuell gewählten Button farblich.
pub fn update_option_highlight(
    settings: Res<GameSettings>,
    mut aspects: Query<(&AspectButton, &mut BackgroundColor), Without<ResolutionButton>>,
    mut resolutions: Query<(&ResolutionButton, &mut BackgroundColor), Without<AspectButton>>,
) {
    for (btn, mut bg) in &mut aspects {
        let col = if btn.0 == settings.ui.aspect { COL_SELECTED } else { COL_BG };
        if bg.0 != col {
            bg.0 = col;
        }
    }
    for (btn, mut bg) in &mut resolutions {
        let col = if btn.0 == settings.ui.window_height { COL_SELECTED } else { COL_BG };
        if bg.0 != col {
            bg.0 = col;
        }
    }
}

pub fn update_value_texts(
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
pub fn update_slider_pos(
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

pub fn go_back_button(
    query: Query<Has<Pressed>, With<GoBackButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    previous: Res<LastState>,
) {
    for pressed in &query {
        if !pressed {
            continue;
        }
        if previous.0 == Some(GameState::InGame) {
            next_state.set(GameState::InGame);
        } else {
            next_state.set(GameState::Welcome);
        }
    }
}