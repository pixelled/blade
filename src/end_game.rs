use bevy::prelude::*;

use super::{AppState};
use crate::component::*;

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::EndGame)
                    .with_system(load_end_game_display)
            )
            .add_system_set(
                SystemSet::on_update(AppState::EndGame)
                    .with_system(end_game_input_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::EndGame)
                    .with_system(despawn_end_game_ui)
            );
    }
}

fn load_end_game_display(mut commands: Commands) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: Color::rgba(0.2, 0.2, 0.2, 0.8).into(),
        ..Default::default()
    }).insert(EndGameUI {});
}

fn end_game_input_system(
    mut app_state: ResMut<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        let _ = app_state.set(AppState::InGame);
    }
}

fn despawn_end_game_ui(
    mut commands: Commands,
    mut queries: Query<Entity, With<EndGameUI>>
) {
    for entity in queries.iter() {
        commands.entity(entity).despawn();
    }
}
