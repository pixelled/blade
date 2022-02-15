use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use crate::RAPIER_TO_LYON;
use crate::shape_mod::Type;

#[derive(Component)]
pub struct StorageBox;

#[derive(Component)]
pub struct BlueprintBox;

#[derive(Default)]
pub struct StorageUIs {
    pub entities: std::vec::Vec<Entity>,
}

#[derive(Default)]
pub struct BlueprintUIs {
    pub entities: std::vec::Vec<Entity>,
    pub res: std::vec::Vec<Entity>,
}

#[derive(Component)]
pub struct StorageUI {
    pub child: Type
}

#[derive(Component)]
pub struct BlueprintUI {
    pub child: Type
}

#[derive(Component)]
pub struct StorageShape;

#[derive(Component)]
pub struct BlueprintShape;

pub fn init_box(extents: Vec2, pos: Vec2) -> ShapeBundle {
    let shape = shapes::Rectangle {
        extents,
        origin: RectangleOrigin::Center
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgba(0.9, 0.9, 0.9, 0.7)),
            outline_mode: StrokeMode::new(Color::rgba(0.7, 0.7, 0.7, 0.7), 5.0),
        },
        Transform {
            translation: Vec3::new(pos.x, pos.y, 1.0),
            ..Default::default()
        },
    )
}