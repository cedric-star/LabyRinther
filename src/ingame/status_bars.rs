use bevy::prelude::*;
use crate::ingame::player::structs::*;
use crate::settings::settings_screen::*;
use crate::util::ui::constants::*;
use crate::components::*;
use crate::ingame::structs::*;
use crate::ingame::entity::structs::*;
use crate::ingame::items::modifier::*;

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
    player: Single<(&Health, &Mana, &Stats), With<Player>>,
    mut health_bar: Query<&mut Text, (With<HealthBar>, Without<SchieldBar>, Without<ManaBar>)>,
    mut armor_bar: Query<&mut Text, (With<SchieldBar>, Without<HealthBar>, Without<ManaBar>)>,
    mut mana_bar: Query<&mut Text, (With<ManaBar>, Without<HealthBar>, Without<SchieldBar>)>,
) {
    let (health, mana, stats) = player.into_inner();
    let armor = stats.get(Stat::Armor);

    for mut text in &mut health_bar {
        text.0 = format!("Health: {:.0}/{:.0}", health.health_points, health.max_health);
    }
    for mut text in &mut armor_bar {
        text.0 = format!("Armor: {:.0}/60", armor);
    }
    for mut text in &mut mana_bar {
        text.0 = format!("Mana: {:.0}/{:.0}", mana.mana_points, mana.max_mana);
    }
}
