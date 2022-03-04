mod animation;
mod bundle;
mod camera;
mod component;
mod end_game;
mod in_game;
mod magic;
mod particle;
mod shape_mod;
mod synthesis;
mod ui;

use bundle::*;
use camera::*;
use end_game::*;
use in_game::*;
use particle::*;
use shape_mod::*;

use crate::animation::AnimationPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

// size(pixels) = RAPIER_SCALE * size(meters)
const RAPIER_TO_BEVY: f32 = 10.0;
const RAPIER_TO_LYON: f32 = 10.0;

const BOUNDARY_HORIZONTAL: f32 = 192.0;
const BOUNDARY_VERTICAL: f32 = 108.0;
const OFFSET_HORIZONTAL: f32 = 50.0;
const OFFSET_VERTICAL: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_state(AppState::Setup)
        .add_plugin(ParticlePlugin)
        .add_plugin(AnimationPlugin)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(setup_game))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(start_game))
        .add_plugin(InGamePlugin)
        .add_plugin(EndGamePlugin)
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
    asset_server: Res<AssetServer>,
    mut config: ResMut<RapierConfiguration>,
) {
    config.gravity = Vec2::new(0.0, 0.0).into();
    config.scale = RAPIER_TO_BEVY;

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera::default());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        texture: asset_server.load("bg.png"),
        ..Default::default()
    });
    commands.spawn_object(Type::Triangle, [5.0, 5.0]);
    commands.spawn_object(Type::Heart, [-5.0, 5.0]);
    // commands.spawn_bundle(BarBundle::new(0.0, 0.0, &asset_server));
    spawn_boundary(&mut commands);

    commands.insert_resource(EntityInRange {
        cur: None,
        prev: None,
    });
    commands.insert_resource(EntityInHand { entity: None });
    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
}

fn start_game(mut app_state: ResMut<State<AppState>>) {
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
