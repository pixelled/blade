mod bundle;
mod component;
mod in_game;
mod end_game;

use bundle::*;
use component::*;
use in_game::*;
use end_game::*;

use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

const TIME_STEP: f32 = 1.0 / 60.0;
const RAPIER_SCALE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_state(AppState::Setup)
        .add_system_set(
            SystemSet::on_enter(AppState::Setup)
                .with_system(setup_game)
                .with_system(start_game)
        )
        .add_plugin(InGamePlugin)
        .add_plugin(EndGamePlugin)
        .run();
}

// fn test_despawn(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, query: Query<Entity, With<Player>>) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         let p = query.single();
//         commands.entity(p).despawn();
//         println!("done");
//     }
// }

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Setup,
    InGame,
    EndGame,
}

// fn setup_menu(mut commands: Commands) {
//
// }
//
// fn menu(mut commands: Commands) {
//
// }
//
// fn cleanup_menu(mut commands: Commands) {
//
// }

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vec2::new(0.0, 0.0).into();
    config.scale = RAPIER_SCALE;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera {});
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
}

fn start_game(mut app_state: ResMut<State<AppState>>) {
    let _ = app_state.set(AppState::InGame).unwrap();
}
