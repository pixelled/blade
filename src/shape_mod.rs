use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use phf::phf_map;

use crate::bundle::*;
use crate::component::*;
use crate::LYON_SCALE;

pub static OBJECTS: &'static [fn(Vec2) -> ObjectBundle] = &[
    square,         // 0
    circle,         // 1
    rect,           // 2
];

pub static TABLE: phf::Map<&'static str, usize> = phf_map! {
    "1+1" => 2,
};

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
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::GRAY, 5.0),
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
                fill_mode: FillMode::color(Color::hsl(0.04, 1.0, 0.6)),
                outline_mode: StrokeMode::new(Color::hsl(0.04, 1.0, 0.4), 5.0),
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
                fill_mode: FillMode::color(Color::YELLOW),
                outline_mode: StrokeMode::new(Color::GRAY, 5.0),
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
