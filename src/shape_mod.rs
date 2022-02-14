use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::bundle::*;
use crate::component::*;
use crate::LYON_SCALE;

use std::collections::HashMap;
use crate::synthesis::BLUEPRINT_SIZE;

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub enum Type {
    Empty,
    Square,
    Circle,
    Rect
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

pub static SHAPES: &'static [(fn(f32) -> ShapeBundle, f32)] = &[
    (empty_shape, 0.0),
    (square_shape, 0.5),        // 1
    (circle_shape, 0.5),        // 2
    (rect_shape, 0.5)           // 3
];

pub static OBJECTS: &'static [fn(Vec2) -> ObjectBundle] = &[
    empty,
    square,         // 1
    circle,         // 2
    rect,           // 3
];

pub fn init_table() -> Vec<(Vec<(Type, usize)>, Type)> {
    use Type::*;
    let table = vec![
        (vec![(Square, 2)], Rect),
        (vec![(Circle, 1)], Rect)
    ];
    table
}

pub fn empty_shape(scale: f32) -> ShapeBundle {
    ShapeBundle::default()
}

pub fn empty(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Empty),
        shape: empty_shape(1.0),
        rigid_body: RigidBodyBundle {
            ..Default::default()
        },
        collider: ColliderBundle {
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
    }
}

pub fn square_shape(scale: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        extents: Vec2::new(2.0, 2.0) * 2.0 * LYON_SCALE * scale,
        origin: RectangleOrigin::Center
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsl(60.0, 1.0, 0.6)),
            outline_mode: StrokeMode::new(Color::hsl(60.0, 1.0, 0.4), 5.0 * scale),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
    )
}

fn square(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Square),
        shape: square_shape(1.0),
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
    }
}

pub fn circle_shape(scale: f32) -> ShapeBundle {
    let shape = shapes::Circle {
        radius: 2.5 * LYON_SCALE * scale,
        center: Vec2::new(0.0, 0.0).clone(),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsl(4.0, 1.0, 0.6)),
            outline_mode: StrokeMode::new(Color::hsl(4.0, 1.0, 0.4), 5.0 * scale),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
    )
}

fn circle(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Circle),
        shape: circle_shape(1.0),
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
    }
}

pub fn rect_shape(scale: f32) -> ShapeBundle {
    let shape = shapes::Rectangle {
        extents: Vec2::new(4.0, 2.0) * 2.0 * LYON_SCALE * scale,
        origin: RectangleOrigin::Center
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsl(60.0, 1.0, 0.6)),
            outline_mode: StrokeMode::new(Color::hsl(60.0, 1.0, 0.4), 5.0 * scale),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
    )
}

fn rect(pos: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Rect),
        shape: rect_shape(1.0),
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
    }
}
