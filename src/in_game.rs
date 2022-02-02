use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::{AppState, TIME_STEP, RAPIER_SCALE};
use crate::component::*;
use crate::bundle::*;
use crate::camera::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_rotate_system)
                    .with_system(player_grab_system)
                    .with_system(player_throw_system)
                    .with_system(player_movement_system)
                    .with_system(move_camera)
                // .with_system(collision_detection_system)
                // .with_system(update_health_display)
                // .with_system(animate)
                // .with_system(update_game_state)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("detect")
                    .with_system(detect_objects_forward)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after("detect")
                    .before("display")

            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("display")
                    .with_system(update_shape_of_detected_objects)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                    .with_system(despawn_the_dead)
            );
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct EntityInRange {
    pub prev: Option<Entity>,
    pub cur: Option<Entity>
}

fn spawn_player(mut commands: Commands) {
    let player = commands.spawn_bundle(PlayerBundle::new(0.0, -10.0)).id();
    let object = commands.spawn_bundle(ObjectBundle::new(10.0, -10.0)).id();
    println!("{:?} {:?}", player, object);
    let axis = Vector::x_axis();
    let joint = PrismaticJoint::new(axis)
        .local_anchor1(point![0.0, 0.0])
        .local_anchor2(point![0.0, 0.0])
        .limit_axis([5.0, 7.0]);
    let entity = commands
        .spawn()
        .insert(JointBuilderComponent::new(joint, player, object))
        .id();
    let joint: JointHandle = entity.handle();
    println!("{:?}", joint);
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

fn player_grab_system(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    entity_in_range: Res<EntityInRange>,
    player_query: Query<Entity, With<Player>>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(entity) = entity_in_range.cur {
            let player = player_query.single();
            let axis = Vector::x_axis();
            let joint = PrismaticJoint::new(axis)
                .local_anchor1(point![0.0, 0.0])
                .local_anchor2(point![0.0, 0.0])
                .limit_axis([5.0, 7.0]);
            let entity = commands.spawn().insert(JointBuilderComponent::new(joint, player, entity)).id();
            let joint: JointHandle = entity.handle();
            println!("{:?}", joint);
            println!("new joint built");
        }
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
        let _hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
        entity_in_range.cur = Some(handle.entity());
        // println!("Entity {:?} hit at point {}", handle.entity(), hit_point);
    }
}

fn update_shape_of_detected_objects(
    mut entity_in_range: ResMut<EntityInRange>,
    mut query: Query<&mut DrawMode>
) {
    if entity_in_range.prev != entity_in_range.cur {
        if let Some(entity) = entity_in_range.prev {
            match query.get_mut(entity) {
                Ok(mode) => {
                    match mode.into_inner() {
                        DrawMode::Outlined { fill_mode: _, outline_mode } => {
                            *outline_mode = StrokeMode::new(Color::GRAY, 5.0);
                        },
                        _ => {}
                    }
                },
                Err(_e) => {}
            }
        }
        if let Some(entity) = entity_in_range.cur {
            match query.get_mut(entity) {
                Ok(mode) => {
                    match mode.into_inner() {
                        DrawMode::Outlined { fill_mode: _, outline_mode } => {
                            *outline_mode = StrokeMode::new(Color::BLACK, 5.0);
                        },
                        _ => {}
                    }
                },
                Err(_e) => {}
            }
        }
    }
    entity_in_range.prev = entity_in_range.cur;
    entity_in_range.cur = None;
}

fn player_throw_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: QuerySet<(
        QueryState<(Entity, &RigidBodyPositionComponent), With<Player>>,
        QueryState<(&mut RigidBodyVelocityComponent, &RigidBodyMassPropsComponent), With<Object>>,
        QueryState<RigidBodyComponentsQueryPayload>
    )>,
    // player_query: Query<(Entity, &RigidBodyPositionComponent), With<Player>>,
    // mut object_query: Query<&mut RigidBodyForcesComponent, With<Object>>,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    // mut rigid_body_query: Query<RigidBodyComponentsQueryPayload>
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let (player, player_pos): (Entity, _) = q.q0().single();
        let rigid_body_handle: RigidBodyHandle = player.handle();
        let rot: UnitComplex<f32> = player_pos.position.rotation;
        let dir_x = rot.cos_angle();
        let dir_y = rot.sin_angle();
        let dir_scale = 100.0;

        use nalgebra::UnitComplex;
        let iter = joint_set.joints_with(rigid_body_handle);
        let mut object_query = q.q1();
        for (h1, h2, j) in iter {
            let (mut obj_vel, obj_mprops) = object_query.get_mut(h2.entity()).unwrap();
            // object.force = Vec2::new(dir_x * dir_scale, dir_y * dir_scale).into();
            obj_vel.apply_impulse(obj_mprops, Vec2::new(dir_x * dir_scale, dir_y * dir_scale).into())
        }

        let mut rigid_body_set = RigidBodyComponentsSet(q.q2());
        joint_set.remove_joints_attached_to_rigid_body(
            rigid_body_handle,
            &mut island_manager,
            &mut rigid_body_set,
        );
        // println!("removed");
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
    queries: Query<(Entity, &Health)>
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
