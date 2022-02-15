mod bundle;
mod component;
mod in_game;
mod end_game;
mod camera;
mod particle;
mod synthesis;
mod shape_mod;
mod ui;

use bundle::*;
use in_game::*;
use end_game::*;
use camera::*;
use particle::*;
use synthesis::*;
use shape_mod::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

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
        .add_system_set(
            SystemSet::on_enter(AppState::Setup)
                .with_system(setup_game)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Setup)
                .with_system(start_game)
        )
        .add_plugin(InGamePlugin)
        .add_plugin(SynthesisPlugin)
        .add_plugin(EndGamePlugin)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Setup,
    InGame,
    EndGame,
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0.0, 0.0).into();
    config.scale = RAPIER_TO_BEVY;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
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
    commands.spawn_bundle(ObjectBundle::new(Vec2::new(5.0, 5.0), Type::Triangle));
    commands.spawn_bundle(ObjectBundle::new(Vec2::new(-5.0, 5.0), Type::Heart));
    commands.spawn_bundle(BarBundle::new(0.0,0.0, &asset_server));
    spawn_boundary(&mut commands);

    spawn_health_bar(&mut commands);
    commands.spawn_bundle(HealthTextBundle::new(&asset_server));

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(10.0)),
                padding: Rect::all(Val::Px(2.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                color: Color::rgb_u8(184, 248, 174).into(),
                ..Default::default()
            }).insert(HealthBarDisplay);
        });
    });

    commands.insert_resource(EntityInRange { cur: None, prev: None });
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
        Vec2::new(0.0, half_m)
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(half_n, OFFSET_VERTICAL),
        Vec2::new(0.0, -half_m)
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(OFFSET_HORIZONTAL, half_m),
        Vec2::new(half_n, 0.0)
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(OFFSET_HORIZONTAL, half_m),
        Vec2::new(-half_n, 0.0)
    ));
}
