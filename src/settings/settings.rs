use bevy::camera::ScalingMode; // Bevy < 0.17: bevy::render::camera::ScalingMode
use bevy::input_focus::tab_navigation::*;
use bevy::input_focus::*;
use bevy::prelude::*;
use bevy::ui_widgets::*;

use crate::components::*;
use crate::settings::aspect_ratio::*;
use crate::settings::settings_screen::*;
use crate::util::ui::buttons::*;

// ---------------------------------------------------------------------------
// Konstanten
// ---------------------------------------------------------------------------

/// Referenzhöhe der Kamera bei camera_mult = 1.0 (in Welt-Pixeln).
pub const BASE_CAMERA_HEIGHT: f32 = 270.0;

/// Auswählbare Fensterhöhen. Die Breite ergibt sich aus dem Seitenverhältnis.
pub const WINDOW_HEIGHTS: [f32; 4] = [720.0, 900.0, 1080.0, 1440.0];

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

    app.init_resource::<GameSettings>()
        .add_systems(OnEnter(GameState::Settings), settings_setup)
        .add_systems(
            Update,
            (
                // UI im Settings-Screen
                (
                    go_back_button,
                    update_slider_pos,
                    update_value_texts,
                    aspect_button_system,
                    resolution_button_system,
                    update_option_highlight,
                    button_visuals,
                    slider_thumb_visuals,
                )
                    .run_if(in_state(GameState::Settings)),
                // Settings auf Kamera / Fenster anwenden (immer aktiv, reagiert nur auf Änderungen)
                (apply_camera, apply_window_resolution),
            ),
        )
        .add_observer(on_setting_change);
}

// ---------------------------------------------------------------------------
// Settings-Structs
// ---------------------------------------------------------------------------

#[derive(Resource, Clone)]
pub struct GameSettings {
    pub ui: UiSettings,
    pub gameplay: GameplaySettings,
    pub derived: DerivedSettings,
}

/// Vom Spieler im Menü einstellbar.
#[derive(Clone)]
pub struct UiSettings {
    pub chunks_loading: i32,
    pub ui_scale: f32,
    /// Sichtweite / Zoom
    pub camera_mult: f32,
    pub aspect: AspectRatio,
    /// Fensterhöhe in Pixeln (Breite folgt aus `aspect`)
    pub window_height: f32,
}

/// Nur im Code / Balancing, NICHT im Menü.
#[derive(Clone)]
pub struct GameplaySettings {
    /// baut Viereck um den Player-Chunk: 1 -> 1 Chunk, 2 -> 3x3, 3 -> 7x7, ...
    pub chunks_enemy_spawnable: f32,
    /// Teiler für die Walk-Border (vorher die Magic Number 1.5)
    pub walk_border_factor: f32,
    pub enemy_waves: EnemyWaveSettings,
}

/// Wird nie direkt geschrieben, nur von `GameSettings::recompute()` berechnet.
#[derive(Clone, Default)]
pub struct DerivedSettings {
    pub camera_width: f32,
    pub camera_height: f32,
    pub player_walk_border_width: f32,
    pub player_walk_border_height: f32,
    pub window_width: f32,
}

#[derive(Clone)]
pub struct EnemyWaveSettings {
    pub free_time: f32, //sekunden zwischen waves
    pub enemy_num: u32, //insgesamt/maximal gespawnte enemies pro wave
}


impl Default for GameSettings {
    fn default() -> Self {
        let mut s = Self {
            ui: UiSettings {
                chunks_loading: 6,
                ui_scale: 6.0,
                camera_mult: 2.0,
                aspect: AspectRatio::R16_9,
                window_height: 720.0,
            },
            gameplay: GameplaySettings {
                chunks_enemy_spawnable: 1.0,
                walk_border_factor: 1.5,
                enemy_waves: EnemyWaveSettings {
                    free_time: 5.0,
                    enemy_num: 5,
                }
            },
            derived: DerivedSettings::default(),
        };
        s.recompute();
        s
    }
}

impl GameSettings {
    /// Einzige Stelle, an der abgeleitete Werte berechnet werden.
    /// Nach jeder Änderung an `ui` oder `gameplay` aufrufen.
    pub fn recompute(&mut self) {
        let aspect = self.ui.aspect.value();
        let mult = self.ui.camera_mult;

        // Kamera: feste Referenzhöhe * Zoom, Breite aus dem Seitenverhältnis
        let cam_h = BASE_CAMERA_HEIGHT * mult;
        let cam_w = cam_h * aspect;

        self.derived.camera_height = cam_h;
        self.derived.camera_width = cam_w;

        // Walk-Border folgt der Kamera.
        // Wie vorher: durch (factor * mult) -> die Border bleibt in Welt-Pixeln
        // konstant, egal wie weit man zoomt. Soll sie mit dem Zoom mitwachsen,
        // nur durch `factor` teilen.
        let f = self.gameplay.walk_border_factor;
        self.derived.player_walk_border_width = cam_w / (f * mult);
        self.derived.player_walk_border_height = cam_h / (f * mult);

        // Fenster
        self.derived.window_width = (self.ui.window_height * aspect).round();
    }
}

// ---------------------------------------------------------------------------
// Anwenden auf Kamera und Fenster
// ---------------------------------------------------------------------------

pub fn apply_camera(settings: Res<GameSettings>, mut cameras: Query<&mut Projection, With<Camera2d>>) {
    if !settings.is_changed() {
        return;
    }
    for mut proj in &mut cameras {
        if let Projection::Orthographic(o) = &mut *proj {
            o.scaling_mode = ScalingMode::Fixed {
                width: settings.derived.camera_width,
                height: settings.derived.camera_height,
            };
        }
    }
}

pub fn apply_window_resolution(settings: Res<GameSettings>, mut windows: Query<&mut Window>) {
    if !settings.is_changed() {
        return;
    }
    for mut window in &mut windows {
        window
            .resolution
            .set(settings.derived.window_width, settings.ui.window_height);
    }
}