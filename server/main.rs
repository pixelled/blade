mod bundle;
mod component;
mod in_game;
mod magic;
mod shape_mod;
mod synthesis;

use bundle::*;
use in_game::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

// size(pixels) = RAPIER_SCALE * size(meters)
const RAPIER_TO_BEVY: f32 = 10.0;

const BOUNDARY_HORIZONTAL: f32 = 192.0;
const BOUNDARY_VERTICAL: f32 = 108.0;
const OFFSET_HORIZONTAL: f32 = 50.0;
const OFFSET_VERTICAL: f32 = 50.0;

fn main() {
    let mut app = App::new();
    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }
    app.add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(InGamePlugin)
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(enter_game))
        .add_system_set(SystemSet::on_exit(AppState::Setup).with_system(setup_game))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Setup,
    InGame,
    EndGame,
}

fn setup_game(
    mut commands: Commands,
    mut config: ResMut<RapierConfiguration>,
) {
    config.gravity = Vec2::new(0.0, 0.0).into();
    config.scale = RAPIER_TO_BEVY;
    spawn_boundary(&mut commands);

    commands.insert_resource(EntityInRange {
        cur: None,
        prev: None,
    });
    commands.insert_resource(EntityInHand { entity: None });
    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
}

fn enter_game(
    mut app_state: ResMut<State<AppState>>,
) {
    let _ = app_state.set(AppState::InGame).unwrap();
}

fn spawn_boundary(commands: &mut Commands) {
    let half_m = BOUNDARY_VERTICAL / 2.0 + OFFSET_VERTICAL;
    let half_n = BOUNDARY_HORIZONTAL / 2.0 + OFFSET_HORIZONTAL;
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(half_n, OFFSET_VERTICAL),
        Vec2::new(0.0, half_m),
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(half_n, OFFSET_VERTICAL),
        Vec2::new(0.0, -half_m),
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(OFFSET_HORIZONTAL, half_m),
        Vec2::new(half_n, 0.0),
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(OFFSET_HORIZONTAL, half_m),
        Vec2::new(-half_n, 0.0),
    ));
}
