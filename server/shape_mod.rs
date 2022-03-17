use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use num_enum::TryFromPrimitive;

use crate::bundle::*;
use crate::component::*;
use Type::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum Type {
    Empty,
    Square,
    Circle,
    Rect,
    Triangle,
    Heart,
    Rust,
}

use std::cmp::Ordering;
impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Less
        } else if self > other {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub static BASIC: &'static [Type] = &[Square, Circle, Triangle];

pub static OBJECTS: &'static [fn(Vec2) -> ObjectBundle] =
    &[empty, square, circle, rect, triangle, heart, rust];

pub fn init_table() -> Vec<(Vec<(Type, usize)>, Type)> {
    let table = vec![
        (vec![(Square, 2)], Rect),
        // (vec![(Circle, 3)], Heart),
        (vec![(Circle, 2), (Triangle, 1)], Heart),
        (vec![(Heart, 2)], Rust),
    ];
    table
}

pub fn empty(_: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Empty),
        rigid_body: RigidBodyBundle {
            ..Default::default()
        },
        collider: ColliderBundle {
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}

fn square(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Square),
        health: Health::new(10),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::cuboid(2.0, 2.0).into(),
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}

fn circle(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Circle),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::ball(2.5).into(),
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}

fn rect(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Rect),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::cuboid(4.0, 2.0).into(),
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}

fn triangle(pos: Vec2) -> ObjectBundle {
    let a = (3.0 as f32).sqrt();
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Triangle),
        health: Health::new(1),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::triangle(
                point![-1.5, 1.5 * a],
                point![-1.5, -1.5 * a],
                point![3.0, 0.0],
            )
            .into(),
            // shape: ColliderShape::cuboid()
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}

fn heart(pos: Vec2) -> ObjectBundle {
    use nalgebra::Isometry2;
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Heart),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::compound(vec![
                (Isometry2::translation(1.5, 1.6), ColliderShape::ball(1.6)),
                (Isometry2::translation(1.5, -1.6), ColliderShape::ball(1.6)),
                (
                    Isometry::translation(0.0, 0.0),
                    ColliderShape::triangle(point![0.6, -3.1], point![0.6, 3.1], point![-3.8, 0.0]),
                ),
            ])
            .into(),
            // (15, 16) 16, (-15, 16) 16, [(6, -31), (6, 31), (-38, 0)]
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}

fn rust(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Rust),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::ball(5.2).into(),
            mass_properties: ColliderMassProps::Density(0.1).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
        ..Default::default()
    }
}
