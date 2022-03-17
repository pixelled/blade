use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::component::*;
use crate::AppState;
use bevy::utils::Duration;

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(heal_timer_system)
                .with_system(heal_system)
                .with_system(heal_animation_system)
                .with_system(magic_timer_system::<Frozen>)
                .with_system(freeze_src_system)
                .with_system(frozen_system)
                .with_system(frozen_animation_system)
                .with_system(magic_timer_system::<Burned>)
                .with_system(burn_src_system)
                .with_system(burned_system)
                .with_system(magic_timer_system::<Paralyzed>)
                .with_system(paralyze_src_system)
                .with_system(paralyzed_system)
                .with_system(paralyzed_animation_system)
                .before("despawn_dead_entities"),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(explode_system)
                .after("collision_detection")
                .before("despawn_dead_entities"),
        );
    }
}

trait MagicWithTimer {
    fn tick(&mut self, duration: Duration) -> &Timer;
}

fn magic_timer_system<T: MagicWithTimer + Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut magic_query: Query<(Entity, &mut T)>,
) {
    for (e, mut magic) in magic_query.iter_mut() {
        if magic.tick(time.delta()).just_finished() {
            commands.entity(e).remove::<T>();
        };
    }
}

/// Heal: heal holder everytime `timer` is finished. May use negative hp for self-damage.
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
    object_query: Query<(&Heal, &Grabbed)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (heal, grabbed) in object_query.iter() {
        if heal.timer.just_finished() {
            let player_entity = grabbed.0;
            let player_pos = player_query.get(player_entity).unwrap();
            // TODO: event
        }
    }
}

/// Sight: expand visual range
#[derive(Component)]
pub struct Sight {
    pub scale: f32,
}

impl Sight {
    pub fn new(scale: f32) -> Self {
        Sight { scale }
    }
}

/// Apply `Frozen` upon hitting players and objects.
#[derive(Component)]
pub struct FreezeSource {
    scale: f32,
    duration: f32,
}

impl FreezeSource {
    pub fn new(scale: f32, duration: f32) -> Self {
        FreezeSource { scale, duration }
    }

    pub fn generate_effect(&self) -> Frozen {
        Frozen {
            scale: self.scale,
            duration: Timer::from_seconds(self.duration, false),
        }
    }
}

/// Slows down speed during some period.
/// Frozen entities don't apply `Frozen` upon collision (things become very complex if applied).
#[derive(Component)]
pub struct Frozen {
    scale: f32,
    duration: Timer,
}

impl MagicWithTimer for Frozen {
    fn tick(&mut self, duration: Duration) -> &Timer {
        self.duration.tick(duration)
    }
}

/// # Bug in Rapier: contact pairs sometimes contain despawned entities.Must check validity before use.
fn freeze_src_system(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    freeze_src_query: Query<(Entity, &FreezeSource)>,
    frozenable_query: Query<(), Or<(With<Player>, With<Object>)>>,
) {
    for (e, freeze_src) in freeze_src_query.iter() {
        for contact_pair in narrow_phase.contacts_with(e.handle()) {
            if contact_pair.has_any_active_contact {
                let other_collider = if contact_pair.collider1 == e.handle() {
                    contact_pair.collider2
                } else {
                    contact_pair.collider1
                };
                let other_e = other_collider.entity();

                if frozenable_query.get(other_e).is_ok() {
                    if freeze_src_query.get(other_e).is_err() {
                        // println!("src: {:?}, frozen: {:?}", e, other_e);
                        commands
                            .entity(other_e)
                            .insert(freeze_src.generate_effect());
                    }
                }
            }
        }
    }
}

fn frozen_system(mut frozen_query: Query<(&mut RigidBodyVelocityComponent, &Frozen)>) {
    for (mut vel, frozen) in frozen_query.iter_mut() {
        vel.linvel *= frozen.scale;
        vel.angvel *= frozen.scale;
    }
}

fn frozen_animation_system(
    frozen_query: Query<&Transform, With<Frozen>>,
) {
    for pos in frozen_query.iter() {
        // TODO: event
    }
}

/// Apply `Burned` upon hitting players and objects.
#[derive(Component)]
pub struct BurnSource {
    dmg: i32,
    duration: f32,
    interval: f32,
}

impl BurnSource {
    pub fn new(dmg: i32, duration: f32, interval: f32) -> Self {
        BurnSource {
            dmg,
            duration,
            interval,
        }
    }

    pub fn generate_effect(&self) -> Burned {
        Burned {
            dmg: self.dmg,
            duration: Timer::from_seconds(self.duration, false),
            interval: Timer::from_seconds(self.interval, true),
        }
    }
}

/// Lose a set amount of hp during some period.
/// Burned entities don't apply `Burned` upon collision (things become very complex if applied).
#[derive(Component)]
pub struct Burned {
    dmg: i32,
    duration: Timer,
    interval: Timer,
}

impl MagicWithTimer for Burned {
    fn tick(&mut self, duration: Duration) -> &Timer {
        self.interval.tick(duration);
        self.duration.tick(duration)
    }
}

/// # Bug in Rapier: contact pairs sometimes contain despawned entities.Must check validity before use.
fn burn_src_system(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    burn_src_query: Query<(Entity, &BurnSource)>,
) {
    for (e, burn_src) in burn_src_query.iter() {
        for contact_pair in narrow_phase.contacts_with(e.handle()) {
            if contact_pair.has_any_active_contact {
                let other_collider = if contact_pair.collider1 == e.handle() {
                    contact_pair.collider2
                } else {
                    contact_pair.collider1
                };
                let other_e = other_collider.entity();
                if burn_src_query.get(other_e).is_err() {
                    commands.entity(other_e).insert(burn_src.generate_effect());
                }
            }
        }
    }
}

fn burned_system(
    mut burned_query: Query<(&Transform, &mut Health, &mut Burned)>,
) {
    for (pos, mut health, burned) in burned_query.iter_mut() {
        if burned.interval.just_finished() {
            health.hp -= burned.dmg;
            // TODO: event
        }
    }
}

/// Apply `Paralyzed` upon hitting players but not upon hitting objects.
#[derive(Component)]
pub struct ParalyzeSource {
    pub duration: f32,
}

impl ParalyzeSource {
    pub fn new(duration: f32) -> Self {
        ParalyzeSource { duration }
    }

    pub fn generate_effect(&self) -> Paralyzed {
        Paralyzed {
            duration: Timer::from_seconds(self.duration, false),
        }
    }
}

/// Unable to move for some period if paralyzed.
#[derive(Component)]
pub struct Paralyzed {
    duration: Timer,
}

impl MagicWithTimer for Paralyzed {
    fn tick(&mut self, duration: Duration) -> &Timer {
        self.duration.tick(duration)
    }
}

/// # Bug in Rapier: contact pairs sometimes contain despawned entities.Must check validity before use.
fn paralyze_src_system(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    paralyze_src_query: Query<(Entity, &ParalyzeSource)>,
    player_query: Query<(), With<Player>>,
) {
    for (e, paralyze_src) in paralyze_src_query.iter() {
        for contact_pair in narrow_phase.contacts_with(e.handle()) {
            if contact_pair.has_any_active_contact {
                let other_collider = if contact_pair.collider1 == e.handle() {
                    contact_pair.collider2
                } else {
                    contact_pair.collider1
                };
                let other_e = other_collider.entity();
                if player_query.get(other_e).is_ok() {
                    commands
                        .entity(other_e)
                        .insert(paralyze_src.generate_effect());
                }
            }
        }
    }
}

fn paralyzed_system(mut paralyzed_query: Query<&mut RigidBodyVelocityComponent, Added<Paralyzed>>) {
    for mut vel in paralyzed_query.iter_mut() {
        vel.linvel = Vec2::ZERO.into();
    }
}

fn paralyzed_animation_system(
    paralyzed_query: Query<&Transform, With<Paralyzed>>,
) {
    for pos in paralyzed_query.iter() {
        // TODO: event
    }
}

/// Explode: deal `dmg` to all entities in a circle of `radius`
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

fn explode_system(
    query_pipeline: Res<QueryPipeline>,
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
            // TODO: animation event
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
