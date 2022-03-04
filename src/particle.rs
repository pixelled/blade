use crate::RAPIER_TO_BEVY;
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

// Reference: https://github.com/cvhariharan/smoke-rs

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScatteringParticles>()
            .add_event::<ExplodeParticles>()
            // .add_startup_system(setup_particles)
            .add_system_set(
                SystemSet::new()
                    .with_system(spawn_particles::<ScatteringParticles>)
                    .with_system(spawn_particles::<ExplodeParticles>)
                    .with_system(update_positions)
                    .with_system(apply_forces)
                    .with_system(kill_particles)
                    .with_system(scale_modifier_system),
            );
    }
}

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Lifetime(i32);

#[derive(Component)]
struct ParticleVel(Vec3);

#[derive(Component)]
struct ParticleAcc(Vec3);

pub trait ParticleEvent {
    fn spawn(&self, commands: &mut Commands);
}

#[derive(Component, Copy, Clone)]
pub struct ScaleModifier(pub f32);

pub struct ScatteringParticles {
    pub pos: Vec3,
    pub num: usize,
    pub color: Color,
    pub vel_scale: f32,
}

impl Default for ScatteringParticles {
    fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 20.0),
            num: 1,
            color: Color::BLACK,
            vel_scale: 1.0,
        }
    }
}

impl ParticleEvent for ScatteringParticles {
    fn spawn(&self, commands: &mut Commands) {
        let mut rng = thread_rng();
        for _ in 0..self.num {
            let dir = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: self.color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            self.pos.x + rng.gen_range(-1.0..1.0),
                            self.pos.y + rng.gen_range(-1.0..1.0),
                            self.pos.z,
                        ),
                        scale: Vec3::new(10.0, 10.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Lifetime(255))
                .insert(ParticleVel(dir.clone() * self.vel_scale))
                .insert(ParticleAcc(Vec3::ZERO));
        }
    }
}

pub struct ExplodeParticles {
    pub pos: Vec3,
    pub radius: f32,
    pub num: usize,
    pub color: Color,
    pub scale_modifier: ScaleModifier,
}

impl ExplodeParticles {
    pub(crate) fn new(pos: Vec3, radius: f32) -> Self {
        ExplodeParticles {
            pos,
            radius: radius * RAPIER_TO_BEVY,
            num: 40,
            color: Color::rgba(1.0, 0.0, 0.0, 1.0),
            scale_modifier: ScaleModifier(0.5),
        }
    }
}

impl ParticleEvent for ExplodeParticles {
    fn spawn(&self, commands: &mut Commands) {
        let mut rng = thread_rng();
        for _ in 0..self.num {
            let r = rng.gen::<f32>().sqrt();
            let theta = rng.gen::<f32>() * 2.0 * PI;
            let dir = Vec3::new(r * theta.cos(), r * theta.sin(), 0.0);
            // let dir = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            let r = self.radius * dir;
            let particle_radius = self.radius - r.length();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: self.color,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(self.pos.x + r.x, self.pos.y + r.y, self.pos.z),
                        scale: Vec3::new(particle_radius, particle_radius, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Lifetime(255))
                .insert(ParticleVel(dir.clone() * 2.0))
                .insert(ParticleAcc(Vec3::ZERO))
                .insert(self.scale_modifier);
        }
    }
}

fn spawn_particles<T: ParticleEvent + Resource>(mut commands: Commands, mut ev: EventReader<T>) {
    for ev in ev.iter() {
        ev.spawn(&mut commands);
    }
}

fn update_positions(mut query: Query<(&mut Transform, &mut ParticleVel, &ParticleAcc)>) {
    for (mut pos, mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0;
        pos.translation += vel.0;
    }
}

fn kill_particles(mut commands: Commands, mut query: Query<(Entity, &mut Lifetime, &mut Sprite)>) {
    for (entity, mut lifetime, mut mode) in query.iter_mut() {
        lifetime.0 -= 3;
        if lifetime.0 <= 0 {
            commands.entity(entity).despawn();
        } else {
            let alpha = lifetime.0 as f32 / 255.0;
            mode.color.set_a(alpha);
        }
    }
}

fn scale_modifier_system(mut query: Query<(&mut Transform, &ScaleModifier)>) {
    for (mut transform, scale_modifier) in query.iter_mut() {
        transform.scale -= scale_modifier.0;
    }
}

fn apply_forces() {}
