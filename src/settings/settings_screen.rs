use bevy::prelude::*;
use bevy::sprite::*;
use bevy::camera::*;
use bevy::window::*;
use crate::world::world_generator::*;
use crate::world::region::*;
use crate::components::*;
use crate::resources::*;
use crate::constants::*;
use std::time::*;

#[derive(Resource)]
pub struct GameSettings {
    pub MOV_SPEED: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            MOV_SPEED: 4.0,
        }
    }
}

//struct SettingsDef {
//    label: &'static str,
//    get: fn(&GameSettings) ->
//}

pub fn settings_plugin(app: &mut App) {
    app
        .add_systems(Update, setting_buttons.run_if(in_state(GameState::Settings)));
}

fn setting_buttons(
    mut commands: Commands,
    s: ResMut<GameSettings>,
) {

}
