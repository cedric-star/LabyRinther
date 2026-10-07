use bevy::prelude::*;
use bevy::app::*;
use crate::components::*;
use crate::util::ui::buttons::*;
use crate::util::ui::constants::*;
use crate::ingame::player::*;
use crate::settings::settings_screen::*;

#[derive(Component)]
pub struct IngameUi;

#[derive(Component)]
pub struct HotBar;

#[derive(Component)]
pub struct HotBarSelector;

pub fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    s: Res<GameSettings>,
) {
    const SCALE: f32 = 5.0; // hier setting einbauen
    let col = Color::srgba(1.0,1.0,1.0,0.1);
    commands.spawn((
        GameEntity,
        IngameUi,
        Node {
            display: Display::Grid,
            width: percent(100),
            height: percent(100),
            grid_template_columns: vec![
                GridTrack::percent(10.0),
                GridTrack::flex(1.0),
                GridTrack::percent(10.0),
            ],
            grid_template_rows: vec![
                GridTrack::percent(10.0),
                GridTrack::flex(1.0),
                GridTrack::percent(10.0),
            ],
            ..default()
        },
        children![
            (
                Node {
                    height: percent(100.0),
                    aspect_ratio: Some(10.0),
                    justify_self: JustifySelf::Center,
                    grid_column: GridPlacement::start(2),
                    grid_row: GridPlacement::start(1),
                    ..default()
                },
                Text::new("My Game"),
                BackgroundColor(col),
            ),
            (
                HotBar,
                Node {
                    width: px(180.0 * s.ui_scale),   // 540
                    height: px(18.0 * s.ui_scale),   // 54
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::End,
                    grid_column: GridPlacement::start(2),
                    grid_row: GridPlacement::start(3),
                    ..default()
                },
                ImageNode::new(asset_server.load("ingame_ui/hotbar.png")),
                children![(
                    HotBarSelector,
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(0.0),
                        top: percent(0.0),
                        width: percent(10.0),    // 54 px = ein Slot
                        height: percent(100.0),
                        ..default()
                    },
                    ImageNode::new(asset_server.load("ingame_ui/selected_slot.png")),
                )],
            ),
        ]
    ));
}

pub fn update_ui(
    player: Query<&Player>,
    mut hot_bar_slot: Query<&mut Node, With<HotBarSelector>>,
) {
    let Ok(player) = player.single() else {
        return; // kein oder mehrere Spieler
    };

    for mut node in &mut hot_bar_slot {
        node.left = percent(player.inventory.selected as f32 * 10.0);
    }
}

pub fn cleanup_ui(
    mut commands: Commands,
    ui: Query<Entity, With<IngameUi>>,
) {
    for entity in &ui {
        commands.entity(entity).despawn();
    }
}
