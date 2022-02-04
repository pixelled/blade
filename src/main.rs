mod bundle;
mod component;
mod in_game;
mod end_game;
mod camera;
mod particle;

use bundle::*;
use in_game::*;
use end_game::*;
use camera::*;
use particle::*;

use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

const TIME_STEP: f32 = 1.0 / 60.0;
const RAPIER_SCALE: f32 = 10.0;
const LYON_SCALE: f32 = 10.0;

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
                .with_system(start_game)
        )
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

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0.0, 0.0).into();
    config.scale = RAPIER_SCALE;

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
    commands.spawn_bundle(ObjectBundle::new(5.0, 5.0));
    commands.spawn_bundle(ObjectBundle::new(-5.0, 5.0));
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
}

fn start_game(mut app_state: ResMut<State<AppState>>) {
    let _ = app_state.set(AppState::InGame).unwrap();
}

fn spawn_boundary(commands: &mut Commands) {
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(112.0, 1.0),
        Vec2::new(0.0, -63.0)
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(112.0, 1.0),
        Vec2::new(0.0, 63.0)
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(1.0, 63.0),
        Vec2::new(112.0, 0.0)
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(1.0, 63.0),
        Vec2::new(-112.0, 0.0)
    ));
}
