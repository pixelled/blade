use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::bundle::*;
use crate::component::*;
use crate::LYON_SCALE;

use std::collections::HashMap;

pub static OBJECTS: &'static [fn(Vec2) -> ObjectBundle] = &[
    square,         // 1
    circle,         // 2
    rect,           // 3
];

pub fn init_table() -> Vec<(Vec<u8>, u8)> {
    vec![
        (vec![1, 1], 3),
        (vec![2], 3)
    ]
}

fn square(pos: Vec2) -> ObjectBundle {
    let shape = shapes::Rectangle {
        extents: Vec2::new(2.0, 2.0) * 2.0 * LYON_SCALE,
        origin: RectangleOrigin::Center
    };

    ObjectBundle {
        object: Object {},
        throwable: Throwable {},
        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::hsl(60.0, 1.0, 0.6)),
                outline_mode: StrokeMode::new(Color::hsl(60.0, 1.0, 0.4), 5.0),
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
        ),
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

fn circle(pos: Vec2) -> ObjectBundle {
    let shape = shapes::Circle {
        radius: 2.5 * LYON_SCALE,
        center: pos.clone(),
    };

    ObjectBundle {
        object: Object {},
        throwable: Throwable {},
        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::hsl(4.0, 1.0, 0.6)),
                outline_mode: StrokeMode::new(Color::hsl(4.0, 1.0, 0.4), 5.0),
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
        ),
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

fn rect(pos: Vec2) -> ObjectBundle {
    let shape = shapes::Rectangle {
        extents: Vec2::new(4.0, 2.0) * 2.0 * LYON_SCALE,
        origin: RectangleOrigin::Center
    };

    ObjectBundle {
        object: Object {},
        throwable: Throwable {},
        shape: GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::hsl(60.0, 1.0, 0.6)),
                outline_mode: StrokeMode::new(Color::hsl(60.0, 1.0, 0.4), 5.0),
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
        ),
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
