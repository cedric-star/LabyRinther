use bevy::prelude::*;
use crate::ingame::player::structs::*;
use crate::settings::settings_screen::*;
use crate::util::ui::constants::*;
use crate::components::*;
use crate::ingame::structs::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct SchieldBar;

#[derive(Component)]
pub struct ManaBar;

pub fn get_bars(
    s: &GameSettings,
    player: &Player,
) -> impl Bundle {

    let scale = s.ui_scale;
    (
        IngameUi,
        GameEntity,
        Node {
            position_type: PositionType::Absolute,
            width: percent(100.0),
            height: percent(70.0),
            left: percent(0.0),
            top: percent(0.0),
            grid_column: GridPlacement::start(3),
            grid_row: GridPlacement::start(1),
            ..default()
        },
        BackgroundColor(BG_COL),
        children![
            (
                HealthBar,
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(100.0),
                    height: percent(20.0),
                    left: px(5.0),
                    top: percent(10.0),
                ..default()
                },
                Text::new("Health"),
                TextFont { font_size: FontSize::Px(4.0 * scale), ..default()},
                TextColor(TXT_COL),
            ),
            (
                SchieldBar,
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(100.0),
                    height: percent(20.0),
                    left: px(5.0),
                    top: percent((100.0 / 3.0) + 10.0),
                ..default()
                },
                Text::new("Schield"),
                TextFont { font_size: FontSize::Px(4.0 * scale), ..default()},
                TextColor(TXT_COL),
            ),
            (
                ManaBar,
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(100.0),
                    height: percent(20.0),
                    left: px(5.0),
                    top: percent((100.0 / 3.0 * 2.0) + 10.0),
                ..default()
                },
                Text::new("Mana"),
                TextFont { font_size: FontSize::Px(4.0 * scale), ..default()},
                TextColor(TXT_COL),
            )
        ],
    )
}

pub fn update_bars(
    player: Query<&Player>,
    mut health_bar: Query<(&HealthBar, &mut Text), (Without<SchieldBar>, Without<ManaBar>)>,
    mut armor_bar: Query<(&SchieldBar, &mut Text), (Without<HealthBar>, Without<ManaBar>)>,
    mut mana_bar: Query<(&ManaBar, &mut Text), (Without<HealthBar>, Without<SchieldBar>)>,
){
    let Ok(player) = player.single() else {
        return;
    };

    for (_, mut text) in &mut health_bar {
        let str = format!("Health: {}/{}", player.health, player.max_health);
        text.clear();
        text.push_str(str.as_str());
    }

    for (_, mut text) in &mut armor_bar {
        let str = format!("Armor: {}/{}", player.armor, 60);
        text.clear();
        text.push_str(str.as_str());
    }

    for(_, mut text) in &mut mana_bar {
        let str = format!("Mana: {}/{}", player.mana, player.max_mana);
        text.clear();
        text.push_str(str.as_str());
    }

}
