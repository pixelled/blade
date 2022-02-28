use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::{thread_rng, Rng};

use super::RAPIER_TO_LYON;
use bevy_prototype_lyon::render::Shape;

// Reference: https://github.com/cvhariharan/smoke-rs

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ParticleEvent>()
            // .add_startup_system(setup_particles)
            .add_system_set(
                SystemSet::new()
                    .with_system(spawn_particles)
                    .with_system(update_positions)
                    .with_system(apply_forces)
                    .with_system(kill_particles)
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

// pub struct ParticleEvent {
//     pub pos: Vec3,
//     pub num: usize,
//     pub color: Color
// }

// impl Default for ParticleEvent {
//     fn default() -> Self {
//         ParticleEvent {
//             color: Color::rgba(0.7, 0.7, 0.7, 1.0),
//             ..Default::default()
//         }
//     }
// }

pub struct ParticleEvent {
    pub pos: Vec3,
    pub num: usize,
    pub color: Color
}

impl Default for ParticleEvent {
    fn default() -> Self {
        ParticleEvent {
            pos: Vec3::ZERO,
            num: 1,
            color: Color::rgba(0.7, 0.7, 0.7, 1.0)
        }
    }
}

// fn setup_particles(mut commands: Commands) {
//     commands.insert_resource(ParticleShape(
//         shapes::Rectangle {
//             extents: Vec2::new(0.5, 0.5) * 2.0 * RAPIER_TO_LYON,
//             origin: RectangleOrigin::Center
//         }
//     ));
// }

fn spawn_particles(
    mut commands: Commands,
    mut ev_despawn: EventReader<ParticleEvent>,
) {
    for ev in ev_despawn.iter() {
        spawn_particle_group(&mut commands, ev.pos, ev.num);
    }
}

fn update_positions(
    mut query: Query<(&mut Transform, &mut ParticleVel, &ParticleAcc)>
) {
    for (mut pos, mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0;
        pos.translation += vel.0;
    }
}

// fn kill_particles(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut Lifetime, &mut DrawMode)>
// ) {
//     for (entity, mut lifetime, mode) in query.iter_mut() {
//         lifetime.0 -= 3;
//         if lifetime.0 <= 0 {
//             commands.entity(entity).despawn();
//         } else {
//             match mode.into_inner() {
//                 DrawMode::Outlined { fill_mode, outline_mode } => {
//                     let alpha = lifetime.0 as f32 / 255.0;
//                     fill_mode.color.set_a(alpha);
//                     outline_mode.color.set_a(alpha);
//                 },
//                 _ => {}
//             }
//         }
//     }
// }

// static PARTICLESHAPE: &'static [shapes::Rectangle] = &[
//     shapes::Rectangle {
//         extents: Vec2::new(0.5, 0.5) * 2.0 * RAPIER_TO_LYON,
//         origin: RectangleOrigin::Center
//     }
// ];

fn kill_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime, &mut Sprite)>
) {
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

fn apply_forces() {

}

fn spawn_particle_group(
    commands: &mut Commands, origin: Vec3, num: usize
) {
    let mut rng = thread_rng();
    // let shape = shapes::Rectangle {
    //     extents: Vec2::new(0.5, 0.5) * 2.0 * RAPIER_TO_LYON,
    //     origin: RectangleOrigin::Center
    // };
    for _ in 0..num {
        commands
            .spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.7, 0.7, 0.7, 1.0),
                        ..Default::default()
                    },
                    transform: Transform {
                            translation: Vec3::new(
                                origin.x + rng.gen_range(-1.0..1.0),
                                origin.y + rng.gen_range(-1.0..1.0),
                                1.0
                            ),
                            scale: Vec3::new(10.0, 10.0, 0.0),
                            ..Default::default()
                        },
                    ..Default::default()
                }
            //     GeometryBuilder::build_as(
            //         &particle_shape.0,
            //         DrawMode::Outlined {
            //             fill_mode: FillMode::color(Color::rgba(0.7, 0.7, 0.7, 1.0)),
            //             outline_mode: StrokeMode::new(Color::GRAY, 1.0),
            //         },
            //         Transform {
            //             translation: Vec3::new(
            //                 origin.x + rng.gen_range(-1.0..1.0),
            //                 origin.y + rng.gen_range(-1.0..1.0),
            //                 1.0
            //             ),
            //             ..Default::default()
            //         },
            //     )
            )
            .insert(Lifetime(255))
            .insert(ParticleVel(Vec3::new(
                rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)))
            .insert(ParticleAcc(Vec3::new(0.0, 0.0, 0.0)));
    }
}
