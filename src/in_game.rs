use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;
use bevy_rapier2d::prelude::*;

use super::{AppState, TIME_STEP};
use crate::bundle::*;
use crate::camera::*;
use crate::component::*;
use crate::magic::*;
use crate::particle::*;
use crate::shape_mod::*;
use crate::synthesis::SynthesisPlugin;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MagicPlugin)
            .add_plugin(SynthesisPlugin)
            .init_resource::<ObjectToPlayer>()
            .insert_resource(TrailTimer(Timer::from_seconds(0.01, true)))
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("general")
                    .with_system(spawn_objects)
                    .with_system(player_rotate_system)
                    .with_system(player_throw_system)
                    .with_system(player_movement_system)
                    .with_system(trail_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(collision_detection)
                    .label("collision_detection"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(despawn_dead_entities)
                    .label("despawn_dead_entities"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_game_state)
                    .after("despawn_dead_entities"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(move_camera)
                    .label("camera")
                    .before("general"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("detection")
                    .with_system(detect_objects_forward),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .after("detection")
                    .before("display")
                    .with_system(player_grab_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .label("display")
                    .with_system(update_shape_of_detected_objects),
            )
            .add_system_set(SystemSet::on_exit(AppState::InGame));
    }
}

#[derive(Debug, Default)]
pub struct EntityInRange {
    pub prev: Option<Entity>,
    pub cur: Option<Entity>,
}

#[derive(Debug, Default)]
pub struct EntityInHand {
    pub entity: Option<Entity>,
}

#[derive(Default)]
pub struct ObjectToPlayer(pub HashMap<Entity, Entity>);

#[derive(Default)]
pub struct SpawnTimer(pub Timer);

fn spawn_objects(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    q: Query<&Object>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if q.iter().len() < 10 {
            let mut rng = thread_rng();
            let idx = rng.gen_range::<u8, _>(0..BASIC.len() as u8);
            commands.spawn_object(BASIC[idx as usize], [-50.0, 50.0]);
        }
    }
}

fn spawn_player(mut commands: Commands, mut entity_in_hand: ResMut<EntityInHand>) {
    let player = commands.spawn_bundle(PlayerBundle::new(0.0, -10.0)).id();
    let object = commands
        .spawn_object(Type::Square, [10.0, -10.0])
        .insert(Grabbed(player))
        .id();
    let axis = Vector::x_axis();
    let joint = PrismaticJoint::new(axis)
        .local_anchor1(point![0.0, 0.0])
        .local_anchor2(point![0.0, 0.0])
        .limit_axis([7.0, 8.0]);
    commands
        .spawn()
        .insert(JointBuilderComponent::new(joint, player, object));
    entity_in_hand.entity = Some(object);
    println!("spawned {:?} {:?}", player, object);
}

fn player_rotate_system(
    windows: Res<Windows>,
    mut player: Query<
        (
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
            &RigidBodyMassPropsComponent,
        ),
        (With<Player>, Without<Paralyzed>),
    >,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        for (player_pos, mut player_vel, _player_mprops) in player.iter_mut() {
            use nalgebra::UnitComplex;
            let size = Vec2::new(window.width() as f32, window.height() as f32);
            let pos = pos - size / 2.0;
            let cursor_rot = UnitComplex::new(pos.y.atan2(pos.x));
            let rot = player_pos.position.rotation.angle_to(&cursor_rot);
            player_vel.angvel = rot / PI * 20.0;
        }
    }
}

fn player_grab_system(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    // mut object_to_player: ResMut<ObjectToPlayer>,
    entity_in_range: Res<EntityInRange>,
    mut entity_in_hand: ResMut<EntityInHand>,
    player_query: Query<Entity, With<Player>>,
) {
    // println!("{:?}", entity_in_hand.entity);
    if entity_in_hand.entity.is_none() && buttons.pressed(MouseButton::Left) {
        if let Some(object_entity) = entity_in_range.cur {
            let player_entity = player_query.single();
            let axis = Vector::x_axis();
            let joint = PrismaticJoint::new(axis)
                .local_anchor1(point![0.0, 0.0])
                .local_anchor2(point![0.0, 0.0])
                .limit_axis([4.0, 7.0]);
            commands.spawn().insert(JointBuilderComponent::new(
                joint,
                player_entity,
                object_entity,
            ));
            entity_in_hand.entity = Some(object_entity);
            commands
                .entity(object_entity)
                .insert(Grabbed(player_entity));
            // object_to_player.0.insert(object_entity, player_entity);
            println!("new joint built with {:?}", object_entity);
        }
    }
}

fn player_throw_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    // mut object_to_player: ResMut<ObjectToPlayer>,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: QuerySet<(
        QueryState<(Entity, &RigidBodyPositionComponent), With<Player>>,
        QueryState<
            (
                &mut RigidBodyVelocityComponent,
                &RigidBodyMassPropsComponent,
            ),
            With<Throwable>,
        >,
        QueryState<RigidBodyComponentsQueryPayload>,
    )>,
) {
    if entity_in_hand.entity.is_some() && keyboard_input.pressed(KeyCode::Space) {
        use nalgebra::UnitComplex;
        let (player, player_pos): (Entity, _) = q.q0().single();
        let rigid_body_handle: RigidBodyHandle = player.handle();
        let rot: UnitComplex<f32> = player_pos.position.rotation;
        let dir_x = rot.cos_angle();
        let dir_y = rot.sin_angle();
        let dir_scale = 1000.0;

        let iter = joint_set.joints_with(rigid_body_handle);
        let mut object_query = q.q1();
        for (_h1, h2, _j) in iter {
            let (mut obj_vel, obj_mprops) = object_query.get_mut(h2.entity()).unwrap();
            // object.force = Vec2::new(dir_x * dir_scale, dir_y * dir_scale).into();
            obj_vel.apply_impulse(
                obj_mprops,
                Vec2::new(dir_x * dir_scale, dir_y * dir_scale).into(),
            )
        }

        let mut rigid_body_set = RigidBodyComponentsSet(q.q2());
        joint_set.remove_joints_attached_to_rigid_body(
            rigid_body_handle,
            &mut island_manager,
            &mut rigid_body_set,
        );
        commands
            .entity(entity_in_hand.entity.unwrap())
            .remove::<Grabbed>();
        // object_to_player.0.remove(&entity_in_hand.entity.unwrap());
        entity_in_hand.entity = None;
    }
}

fn detect_objects_forward(
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    player_query: Query<&RigidBodyPositionComponent, With<Player>>,
    throwable_query: Query<&Throwable>,
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

    if let Some((handle, toi)) =
        query_pipeline.cast_ray(&collider_set, &ray, max_toi, solid, groups, filter)
    {
        if throwable_query.get(handle.entity()).is_ok() {
            let _hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
            entity_in_range.cur = Some(handle.entity());
            // println!("Entity {:?} hit at point {}", handle.entity(), hit_point);
        }
    }
}

fn update_shape_of_detected_objects(
    mut entity_in_range: ResMut<EntityInRange>,
    mut query: Query<(&mut DrawMode, &Throwable)>,
) {
    if entity_in_range.prev != entity_in_range.cur {
        if let Some(entity) = entity_in_range.prev {
            match query.get_mut(entity) {
                Ok((mode, id)) => match mode.into_inner() {
                    DrawMode::Outlined {
                        fill_mode: _,
                        outline_mode,
                    } => {
                        let c = outline_mode.color.as_hlsa_f32();
                        *outline_mode = StrokeMode::new(
                            Color::hsl(c[0], c[1], 0.4).into(),
                            5.0 * SCALE[id.0 as usize],
                        );
                    }
                    _ => {}
                },
                Err(_e) => {}
            }
        }
        if let Some(entity) = entity_in_range.cur {
            match query.get_mut(entity) {
                Ok((mode, id)) => match mode.into_inner() {
                    DrawMode::Outlined {
                        fill_mode: _,
                        outline_mode,
                    } => {
                        let c = outline_mode.color.as_hlsa_f32();
                        *outline_mode = StrokeMode::new(
                            Color::hsl(c[0], c[1], 0.1).into(),
                            5.0 * SCALE[id.0 as usize],
                        );
                    }
                    _ => {}
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
    mut player: Query<
        (&RigidBodyVelocityComponent, &mut RigidBodyForcesComponent),
        (With<Player>, Without<Paralyzed>),
    >,
) {
    if *app_state.current() == AppState::EndGame {
        return;
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
    for (player_vel, mut player_forces) in player.iter_mut() {
        let player_vel: Vec2 = player_vel.linvel.into();

        let dir_scale = 2000.0;

        if player_vel.length() > 0.01 {
            let friction_dir = player_vel.normalize();
            let friction = friction_dir * 600.0;
            // println!("{}", player_vel.linvel);
            player_forces.force =
                (Vec2::new(dir_x * dir_scale, dir_y * dir_scale) - friction).into();
        } else {
            player_forces.force = Vec2::new(dir_x * dir_scale, dir_y * dir_scale).into();
        }
    }
}

/// # Safety: h1 and h2 should be different
fn collision_detection(
    mut contact_events: EventReader<ContactEvent>,
    q: Query<(&mut Health, &mut Dmg, &RigidBodyVelocityComponent)>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                unsafe {
                    // Todo: check why h1 exists but h1.entity() doesn't exist
                    let query1 = q.get_unchecked(h1.entity());
                    let query2 = q.get_unchecked(h2.entity());
                    if query1.is_ok() && query2.is_ok() {
                        let (mut health1, dmg1, vel1): (
                            Mut<Health>,
                            Mut<Dmg>,
                            &RigidBodyVelocityComponent,
                        ) = query1.unwrap();
                        let (mut health2, dmg2, vel2) = query2.unwrap();
                        let rel_linvel = vel1.linvel - vel2.linvel;
                        if rel_linvel.norm() > 80.0 {
                            health1.hp -= dmg2.0;
                            health2.hp -= dmg1.0;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

struct TrailTimer(Timer);

fn trail_system(
    mut ev_despawn: EventWriter<DespawnParticles>,
    time: Res<Time>,
    mut timer: ResMut<TrailTimer>,
    q: Query<&Transform, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let player_pos = q.single();
        ev_despawn.send(DespawnParticles {
            pos: Vec3::from([player_pos.translation.x, player_pos.translation.y, 1.0]),
            num: 5,
            color: Color::rgba(0.7, 0.7, 0.7, 1.0),
        });
    }
}

fn despawn_dead_entities(
    mut commands: Commands,
    mut particle_ev: EventWriter<DespawnParticles>,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    mut entity_in_hand: ResMut<EntityInHand>,
    q0: Query<
        (
            Entity,
            &Health,
            &Transform,
            Option<&Grabbed>,
            Option<&Player>,
        ),
        Without<Undead>,
    >,
    q1: Query<RigidBodyComponentsQueryPayload>,
) {
    let mut rigid_body_set = RigidBodyComponentsSet(q1);
    for (e, health, pos, grabbed, player) in q0.iter() {
        if health.hp <= 0 {
            particle_ev.send(DespawnParticles::new(Vec3::from([
                pos.translation.x,
                pos.translation.y,
                15.0,
            ])));
            if player.is_some() || grabbed.is_some() {
                let rigid_body_handle: RigidBodyHandle = e.handle();
                joint_set.remove_joints_attached_to_rigid_body(
                    rigid_body_handle,
                    &mut island_manager,
                    &mut rigid_body_set,
                );
                entity_in_hand.entity = None;
            }
            if player.is_some() {
                println!("player dead: {:?}", health.hp);
            }
            commands.entity(e).despawn();
        }
    }
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
