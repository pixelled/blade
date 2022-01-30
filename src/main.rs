mod bundle;

use bundle::*;

use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy_rapier2d::prelude::*;
use bevy::reflect::erased_serde::private::serde::__private::de::EnumDeserializer;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;
use bevy_prototype_lyon::prelude::ShapePlugin;

const TIME_STEP: f32 = 1.0 / 60.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(setup_game).with_system(start_game))
        // .add_system_set(SystemSet::on_update(AppState::InGame).with_system(update1))
        // .add_system_set(SystemSet::on_update(AppState::EndGame).with_system(update2))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // .with_system(test_despawn)
                .with_system(player_rotate_system)
                .with_system(detect_objects_forward)
                .with_system(player_throw_system)
                .with_system(player_movement_system)
                .with_system(focus_camera)
                // .with_system(collision_detection_system)
                // .with_system(update_health_display)
                // .with_system(animate)
                // .with_system(update_game_state)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame)
                .with_system(despawn_the_dead)
        )
        .add_system_set(SystemSet::on_enter(AppState::EndGame))
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
        )
        .run();
}

// fn test_despawn(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>, query: Query<Entity, With<Player>>) {
//     if keyboard_input.just_pressed(KeyCode::Space) {
//         let p = query.single();
//         commands.entity(p).despawn();
//         println!("done");
//     }
// }

fn test() {
    println!("Exit!");
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Setup,
    InGame,
    EndGame,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum EndGameState {
    Disabled,
    Stats
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health {
    hp: i32,
}

fn setup_menu(mut commands: Commands) {

}

fn menu(mut commands: Commands) {

}

fn cleanup_menu(mut commands: Commands) {

}

const RAPIER_SCALE: f32 = 10.0;

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

#[derive(Default)]
struct EntityInRange {
    prev: Option<Entity>,
    cur: Option<Entity>
}

fn start_game(mut app_state: ResMut<State<AppState>>) {
    let _ = app_state.set(AppState::InGame).unwrap();
}

fn spawn_player(mut commands: Commands) {
    let player = commands.spawn_bundle(PlayerBundle::new(0.0, 0.0)).id();
    let object = commands.spawn_bundle(ObjectBundle::new(5.0, 0.0)).id();
    let joint = RevoluteJoint::new()
        .local_anchor1(point![0.0, 0.0])
        .local_anchor2(point![5.0, 0.0]);
    commands.spawn().insert(JointBuilderComponent::new(joint, player, object));
    println!("spawned");
}

fn player_rotate_system(
    windows: Res<Windows>,
    mut player: Query<&mut RigidBodyPositionComponent, With<Player>>
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let mut player_pos = player.single_mut();
        use nalgebra::UnitComplex;
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let pos = pos - size / 2.0;
        let angle = pos.y.atan2(pos.x);
        player_pos.position.rotation = UnitComplex::new(angle);
    }
}

fn detect_objects_forward(
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    player_query: Query<&RigidBodyPositionComponent, With<Player>>,
    mut entity_in_range: ResMut<EntityInRange>,
) {
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let player = player_query.single();
    let r = player.position.rotation;
    let re = r.cos_angle();
    let im = r.sin_angle();
    let dir = Vec2::new(re, im).into();

    // origin is right outside the player's forward boundary
    let x = player.position.translation.x + 2.1 * re;
    let y = player.position.translation.y + 2.1 * im;
    let origin = Vec2::new(x, y).into();

    let ray = Ray::new(origin, dir);
    let max_toi = 4.0;
    let solid = false;
    let groups = InteractionGroups::all();
    let filter = None;

    if let Some((handle, toi)) = query_pipeline.cast_ray(
        &collider_set, &ray, max_toi, solid, groups, filter
    ) {
        let hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
        entity_in_range.cur = Some(handle.entity());
        // println!("Entity {:?} hit at point {}", handle.entity(), hit_point);
    }
}

fn player_throw_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    mut rigid_body_query: Query<RigidBodyComponentsQueryPayload>
) {
    let mut rigid_body_set = RigidBodyComponentsSet(rigid_body_query);
    if keyboard_input.pressed(KeyCode::Space) {
        // println!("{}", 1);
        let mut handles = vec![];
        for (h, joint) in joint_set.iter() {
            handles.push(h);
            // print!("{:?} {:?} : ", joint.body1, joint.body2);
        }
        for h in handles.into_iter() {
            joint_set.remove(h, &mut island_manager, &mut rigid_body_set, false);
        }
        // print!("\n");
    }
}

fn player_movement_system(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&RigidBodyVelocityComponent, &mut RigidBodyForcesComponent, With<Player>)>,
) {
    if *app_state.current() == AppState::EndGame {
        return
    }
    let (mut dir_x, mut dir_y) = (0.0, 0.0);
    if keyboard_input.pressed(KeyCode::A) {
        dir_x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        dir_x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::W) {
        dir_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        dir_y -= 1.0;
    }
    let (player_vel, mut player_forces, _) = player.single_mut();
    let player_vel: Vec2 = player_vel.linvel.into();

    let dir_scale = 2000.0;

    if player_vel.length() > 0.01 {
        let friction_dir = player_vel.normalize();
        let friction = friction_dir * 600.0;
        // println!("{}", player_vel.linvel);
        player_forces.force = (Vec2::new(dir_x * dir_scale, dir_y * dir_scale) - friction).into();
    } else {
        player_forces.force = Vec2::new(dir_x * dir_scale, dir_y * dir_scale).into();
    }
}

fn focus_camera(
    app_state: Res<State<AppState>>,
    player: Query<&RigidBodyPositionComponent, With<Player>>,
    mut camera: Query<&mut Transform, With<MainCamera>>
) {
    if *app_state.current() == AppState::EndGame {
        return
    }
    let position = player.single();
    let translation = &position.position.translation;
    let mut camera_transform = camera.single_mut();
    camera_transform.translation.x = translation.vector.x * RAPIER_SCALE;
    camera_transform.translation.y = translation.vector.y * RAPIER_SCALE;
}

fn collision_detection_system(
    mut contact_events: EventReader<ContactEvent>,
    mut health_queries: Query<&mut Health>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                if let Ok(mut health) = health_queries.get_mut(h1.entity()) {
                    health.hp -= 10;
                }
                if let Ok(mut health) = health_queries.get_mut(h2.entity()) {
                    health.hp -= 10;
                }
            },
            _ => {}
        }
    }
}

fn despawn_the_dead(
    mut commands: Commands,
    mut queries: Query<(Entity, &Health)>
) {
    println!("despawn");
    for (entity, health) in queries.iter() {
        if health.hp <= 0 {
            println!("{:?}", commands.entity(entity).id());

            commands.entity(entity).despawn();
            println!("despawn done");
        }
    }
}

fn update_health_display (
    app_state: Res<State<AppState>>,
    mut query: Query<&mut Text>,
    player_health: Query<&Health, With<Player>>,
    mut health_bar: Query<&mut Style, With<HealthBarDisplay>>,
    mut health_bar_component: Query<&mut HealthBarDisplayComponent>,
) {
    if *app_state.current() == AppState::EndGame {
        return
    }
    let mut text = query.single_mut();
    let health = player_health.single();
    text.sections[1].value = format!("{}", health.hp);

    let mut health_bar = health_bar.single_mut();
    health_bar.size = Size::new(Val::Percent(health.hp as f32), Val::Percent(80.0));

    let mut health_bar = health_bar_component.single_mut();
    health_bar.cur_percent = health.hp as f32;
}

fn update_game_state(
    mut app_state: ResMut<State<AppState>>,
    player_health: Query<&Health, With<Player>>,
) {
    match app_state.current() {
        AppState::InGame => {
            let health = player_health.single();
            if health.hp <= 0 {
                app_state.set(AppState::EndGame).unwrap();
            }
        }
        _ => {}
    }
}

fn animate(mut health_bar_component: Query<(&mut Transform, &mut HealthBarDisplayComponent)>) {
    let (mut transform, mut health_bar) = health_bar_component.single_mut();
    health_bar.animate(transform);
}

#[derive(Component)]
struct EndGameUI;

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
        app_state.set(AppState::InGame);
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
