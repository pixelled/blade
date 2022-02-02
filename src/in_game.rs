use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::{AppState, TIME_STEP};
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
                    .with_system(player_throw_system)
                    .with_system(player_movement_system)
                    .with_system(move_camera)
                    .with_system(collision_detection)
                    .with_system(update_game_state)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("detection")
                    .with_system(detect_objects_forward)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after("detection")
                    .before("display")
                    .with_system(player_grab_system)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("display")
                    .with_system(update_shape_of_detected_objects)
                    .with_system(update_health_display)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("animation")
                    .with_system(animate)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame)
                    .with_system(despawn_dead_entities)
            );
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct EntityInRange {
    pub prev: Option<Entity>,
    pub cur: Option<Entity>
}

#[derive(Debug)]
#[derive(Default)]
pub struct EntityInHand {
    pub entity: Option<Entity>
}

fn spawn_player(
    mut commands: Commands,
    mut entity_in_hand: ResMut<EntityInHand>
) {
    let player = commands.spawn_bundle(PlayerBundle::new(0.0, -10.0)).id();
    let object = commands.spawn_bundle(ObjectBundle::new(10.0, -10.0)).id();
    let axis = Vector::x_axis();
    let joint = PrismaticJoint::new(axis)
        .local_anchor1(point![0.0, 0.0])
        .local_anchor2(point![0.0, 0.0])
        .limit_axis([5.0, 7.0]);
    commands
        .spawn()
        .insert(JointBuilderComponent::new(joint, player, object));
    entity_in_hand.entity = Some(object);
    println!("spawned {:?} {:?}", player, object);
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
    mut entity_in_hand: ResMut<EntityInHand>,
    player_query: Query<Entity, With<Player>>,
) {
    if entity_in_hand.entity.is_none() && buttons.pressed(MouseButton::Left) {
        if let Some(entity) = entity_in_range.cur {
            let player = player_query.single();
            let axis = Vector::x_axis();
            let joint = PrismaticJoint::new(axis)
                .local_anchor1(point![0.0, 0.0])
                .local_anchor2(point![0.0, 0.0])
                .limit_axis([5.0, 7.0]);
            commands.spawn().insert(JointBuilderComponent::new(joint, player, entity));
            entity_in_hand.entity = Some(entity);
            println!("new joint built with {:?}", entity);
        }
    }
}

fn player_throw_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: QuerySet<(
        QueryState<(Entity, &RigidBodyPositionComponent), With<Player>>,
        QueryState<(&mut RigidBodyVelocityComponent, &RigidBodyMassPropsComponent), With<Object>>,
        QueryState<RigidBodyComponentsQueryPayload>
    )>,
) {
    if entity_in_hand.entity.is_some() && keyboard_input.pressed(KeyCode::Space) {
        let (player, player_pos): (Entity, _) = q.q0().single();
        let rigid_body_handle: RigidBodyHandle = player.handle();
        let rot: UnitComplex<f32> = player_pos.position.rotation;
        let dir_x = rot.cos_angle();
        let dir_y = rot.sin_angle();
        let dir_scale = 1000.0;

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
        entity_in_hand.entity = None;
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

fn collision_detection(
    mut contact_events: EventReader<ContactEvent>,
    mut health_queries: Query<&mut Health>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                if let Ok(mut health) = health_queries.get_mut(h1.entity()) {
                    health.hp -= 5;
                }
                if let Ok(mut health) = health_queries.get_mut(h2.entity()) {
                    health.hp -= 5;
                }
            },
            _ => {}
        }
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: QuerySet<(
        QueryState<(Entity, &Health), With<Player>>,
        QueryState<RigidBodyComponentsQueryPayload>
    )>,
) {
    let (player_entity, player_health): (Entity, &Health) = q.q0().single();
    if player_health.hp <= 0 {
        let rigid_body_handle: RigidBodyHandle = player_entity.handle();

        let mut rigid_body_set = RigidBodyComponentsSet(q.q1());
        joint_set.remove_joints_attached_to_rigid_body(
            rigid_body_handle,
            &mut island_manager,
            &mut rigid_body_set,
        );

        commands.entity(player_entity).despawn();
        entity_in_hand.entity = None;
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

fn animate(mut health_bar_component: Query<(&mut Transform, &mut HealthBarDisplayComponent)>) {
    let (mut transform, mut health_bar) = health_bar_component.single_mut();
    health_bar.animate(transform);
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
