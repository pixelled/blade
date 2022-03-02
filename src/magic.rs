use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::camera::*;
use crate::component::*;
use crate::particle::*;
use crate::AppState;

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(heal_timer_system)
                .with_system(heal_system)
                .with_system(sight_system)
                .with_system(heal_animation_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(explode_system)
                .after("collision_detection")
                .before("despawn_dead_entities"),
        );
    }
}

/// Heal when timer is finished. Use negative hp for self-damage.
#[derive(Component)]
pub struct Heal {
    pub hp: i32,
    pub timer: Timer,
}

impl Heal {
    pub fn new(hp: i32, interval: f32) -> Self {
        Heal {
            hp,
            timer: Timer::from_seconds(interval, true),
        }
    }
}

#[derive(Component)]
pub struct Sight {
    pub scale: f32,
}

impl Sight {
    pub fn new(scale: f32) -> Self {
        Sight { scale }
    }
}

#[derive(Component)]
pub struct Freeze {}

#[derive(Component)]
pub struct Burn {}

#[derive(Component)]
pub struct Paralyze {}

#[derive(Component)]
pub struct Explode {
    pub radius: f32,
    pub dmg: i32,
}

impl Explode {
    pub fn new(radius: f32, dmg: i32) -> Self {
        Explode { radius, dmg }
    }
}

fn heal_timer_system(time: Res<Time>, mut magic_query: Query<&mut Heal, With<Grabbed>>) {
    for mut heal in magic_query.iter_mut() {
        heal.timer.tick(time.delta());
    }
}

fn heal_system(object_query: Query<(&Heal, &Grabbed)>, mut player_query: Query<&mut Health>) {
    for (heal, grabbed) in object_query.iter() {
        if heal.timer.just_finished() {
            let player_entity = grabbed.0;
            let mut player_health = player_query.get_mut(player_entity).unwrap();
            player_health.heal(heal.hp);
        }
    }
}

fn heal_animation_system(
    mut ev_particle: EventWriter<DespawnParticles>,
    object_query: Query<(&Heal, &Grabbed)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (heal, grabbed) in object_query.iter() {
        if heal.timer.just_finished() {
            let player_entity = grabbed.0;
            let player_pos = player_query.get(player_entity).unwrap();
            ev_particle.send(DespawnParticles {
                pos: Vec3::from([player_pos.translation.x, player_pos.translation.y, 20.0]),
                num: 3,
                color: Color::rgb_u8(184, 248, 174),
            });
        }
    }
}

fn sight_system(
    mut camera: Query<(&mut OrthographicProjection, &MainCamera)>,
    object_query: Query<&Sight, With<Grabbed>>,
) {
    let (mut camera_config, camera) = camera.single_mut();
    if object_query.is_empty() {
        camera_config.scale = (camera_config.scale - camera.speed_z).max(1.0);
    } else {
        let sight = object_query.single();
        camera_config.scale = (camera_config.scale + camera.speed_z).min(sight.scale);
    }
}

fn freeze_system() {
    todo!()
}

fn burn_system() {
    todo!()
}

fn paralyze_system() {
    todo!()
}

fn explode_system(
    query_pipeline: Res<QueryPipeline>,
    mut ev_explosion: EventWriter<ExplodeParticles>,
    collider_query: QueryPipelineColliderComponentsQuery,
    explode_query: Query<(Entity, &Explode, &Transform)>,
    mut health_query: Query<&mut Health>,
    mut rigid_bodies: Query<(
        &RigidBodyPositionComponent,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
    )>,
) {
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
    for (explode_entity, explode, pos) in explode_query.iter() {
        let health = health_query.get(explode_entity).unwrap();
        if health.hp <= 0 {
            // Animation
            ev_explosion.send(ExplodeParticles::new(
                Vec3::new(pos.translation.x, pos.translation.y, 20.0),
                explode.radius,
            ));
            // Explosion on surrounding objects
            let (explode_pos, _, _) = rigid_bodies.get(explode_entity).unwrap();
            let explode_shape = ColliderShape::ball(explode.radius);
            let groups = InteractionGroups::all();
            let filter = None;
            query_pipeline.intersections_with_shape(
                &collider_set,
                &explode_pos.position,
                explode_shape.as_ref(),
                groups,
                filter,
                |handle| {
                    let e = handle.entity();
                    let mut health = health_query.get_mut(e).unwrap();
                    health.hp -= explode.dmg;
                    // TODO: repulsion from explosion center
                    // let (rb_pos, mut rb_vel, rb_mprops) = rigid_bodies.get_mut(e).unwrap();
                    // let dist = rb_pos.position.translation.vector - explode_pos.position.translation.vector;
                    // let impulse = Vec2::new(100.0 / dist.x, 100.0 / dist.y).into();
                    // rb_vel.apply_impulse(rb_mprops, impulse);
                    true
                },
            );
        }
    }
}
