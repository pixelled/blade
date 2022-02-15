use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::bundle::*;
use crate::component::*;
use crate::RAPIER_TO_LYON;

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub enum Type {
    Empty,
    Square,
    Circle,
    Rect,
    Triangle,
    Heart,
}

pub enum Usage {
    World,
    Storage,
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

pub static SHAPES: &'static [fn(Usage) -> ShapeBundle] = &[
    empty_shape,
    square_shape,
    circle_shape,
    rect_shape,
    triangle_shape,
    heart_shape,
];

pub static OBJECTS: &'static [fn(Vec2) -> ObjectBundle] = &[
    empty,
    square,
    circle,
    rect,
    triangle,
    heart
];

pub fn init_table() -> Vec<(Vec<(Type, usize)>, Type)> {
    use Type::*;
    let table = vec![
        (vec![(Square, 2)], Rect),
        (vec![(Circle, 3)], Heart),
        (vec![(Circle, 2), (Triangle, 1)], Heart)
    ];
    table
}

pub fn empty_shape(_: Usage) -> ShapeBundle {
    ShapeBundle::default()
}

pub fn empty(_: Vec2) -> ObjectBundle {
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Empty),
        shape: empty_shape(Usage::World),
        rigid_body: RigidBodyBundle {
            ..Default::default()
        },
        collider: ColliderBundle {
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
    }
}

pub fn square_shape(usage: Usage) -> ShapeBundle {
    let scale = match usage {
        Usage::World => 1.0,
        Usage::Storage => 0.5,
    };
    let shape = shapes::Rectangle {
        extents: Vec2::new(2.0, 2.0) * 2.0 * RAPIER_TO_LYON * scale,
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
        shape: square_shape(Usage::World),
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

pub fn circle_shape(usage: Usage) -> ShapeBundle {
    let scale = match usage {
        Usage::World => 1.0,
        Usage::Storage => 0.5,
    };
    let shape = shapes::Circle {
        radius: 2.5 * RAPIER_TO_LYON * scale,
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
        shape: circle_shape(Usage::World),
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

pub fn rect_shape(usage: Usage) -> ShapeBundle {
    let scale = match usage {
        Usage::World => 1.0,
        Usage::Storage => 0.3,
    };
    let shape = shapes::Rectangle {
        extents: Vec2::new(4.0, 2.0) * 2.0 * RAPIER_TO_LYON * scale,
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
        shape: rect_shape(Usage::World),
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

fn triangle_shape(usage: Usage) -> ShapeBundle {
    let scale = match usage {
        Usage::World => 1.0,
        Usage::Storage => 0.4,
    };
    let svg_path_string = match usage {
        Usage::World => "M -15 -26 L -15 26 L 30 0 L -15 -26 L -15 26".to_owned(),
        Usage::Storage => "M -6 -10.4 L -6 10.4 L 12 0 L -6 -10.4 L -6 10.4".to_owned(),
    };
    GeometryBuilder::build_as(
        &shapes::SvgPathShape {
            svg_path_string,
            svg_doc_size_in_px: Vec2::new(0.,0.)
        },
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsl(200.0, 1.0, 0.6)),
            outline_mode: StrokeMode::new(Color::hsl(200.0, 1.0, 0.4), 5.0 * scale),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
    )
}

fn triangle(pos: Vec2) -> ObjectBundle {
    let a = (3.0 as f32).sqrt();
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Triangle),
        shape: triangle_shape(Usage::World),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::triangle(
                point![-1.5, 1.5 * a], point![-1.5, -1.5 * a], point![3.0, 0.0]
            ).into(),
            // shape: ColliderShape::cuboid()
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
    }
}

fn heart_shape(usage: Usage) -> ShapeBundle {
    let scale = match usage {
        Usage::World => 1.0,
        Usage::Storage => 0.3,
    };
    let svg_path_string = match usage {
        Usage::World => "M 6.476 31.244 C 37.726 37.494 37.726 -0.006 12.726 -0.006 C 37.726 -0.006 37.726 -37.506 6.476 -31.256 C -12.274 -25.006 -18.524 -6.256 -37.274 -0.006 C -18.524 6.244 -12.274 24.994 6.476 31.244".to_owned(),
        Usage::Storage => "M 1.9428 9.3732 C 11.3178 11.2482 11.3178 -0.0018 3.8178 -0.0018 C 11.3178 -0.0018 11.3178 -11.2518 1.9428 -9.3768 C -3.6822 -7.5018 -5.5572 -1.8768 -11.1822 -0.0018 C -5.5572 1.8732 -3.6822 7.4982 1.9428 9.3732".to_owned(),
    };
    let shape = shapes::SvgPathShape {
        svg_path_string,
        svg_doc_size_in_px: Vec2::new(0., 0.)
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::hsl(344.0, 1.0, 0.6)),
            outline_mode: StrokeMode::new(Color::hsl(344.0, 1.0, 0.4), 5.0 * scale),
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
    )
}

fn heart(pos: Vec2) -> ObjectBundle {
    use nalgebra::Isometry2;
    ObjectBundle {
        object: Object {},
        throwable: Throwable(Type::Heart),
        shape: heart_shape(Usage::World),
        rigid_body: RigidBodyBundle {
            position: (pos.clone(), 0.0).into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            shape: ColliderShape::compound(vec![
                (Isometry2::translation(1.5, 1.6), ColliderShape::ball(1.6)),
                (Isometry2::translation(1.5, -1.6), ColliderShape::ball(1.6)),
                (Isometry::translation(0.0, 0.0), ColliderShape::triangle(
                    point![0.6, -3.1], point![0.6, 3.1], point![-3.8, 0.0]
                ))
            ]
            ).into(),
            // (15, 16) 16, (-15, 16) 16, [(6, -31), (6, 31), (-38, 0)]
            mass_properties: ColliderMassProps::Density(0.4).into(),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..Default::default()
        },
        sync: RigidBodyPositionSync::Discrete,
    }
}
