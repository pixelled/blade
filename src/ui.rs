use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use crate::RAPIER_TO_LYON;
use crate::shape_mod::{Type, square_shape, Usage};

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

#[derive(Default)]
pub struct RecipeTableUi(pub Option<Entity>);

#[derive(Component)]
pub struct RecipeBoxUi(pub Option<Entity>);

#[derive(Component)]
pub struct RecipeShape;

pub fn init_ui(mut commands: Commands) {
    let e = commands.spawn_bundle(square_shape(Usage::Storage)).insert(RecipeShape {}).id();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0.6, 0.6, 0.6, 0.1).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                    margin: Rect::all(Val::Px(10.0)),
                    border: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                color: Color::rgba(0.6, 0.6, 0.6, 0.3).into(),
                ..Default::default()
            }).insert(RecipeBoxUi(Some(e)));
        });
}

use crate::camera::*;

pub fn update_ui(
    wnds: Res<Windows>,
    mut q: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<(&Transform, &GlobalTransform, &RecipeBoxUi)>,
        QueryState<&mut Transform, With<RecipeShape>>
    )>
) {
    let wnd = wnds.get_primary().unwrap();
    let camera_pos = q.q0().single();
    // println!("{}", camera_pos.translation);
    let mut x = camera_pos.translation.x;
    let mut y = camera_pos.translation.y;

    let (pos, global_pos, ui) = q.q1().single();
    // println!("{} {}", pos.translation, global_pos.translation);
    x -= wnd.width() as f32 / 2.0 - global_pos.translation.x;
    y -= wnd.height() as f32 / 2.0 - global_pos.translation.y;

    let mut q2 = q.q2();
    let mut pos = q2.single_mut();
    // println!("{} {}", x, y);
    pos.translation.x = x;
    pos.translation.y = y;

    // let (pos, global_pos) = q.q2().single_mut();
    // println!("{} {}", pos.translation, global_pos.translation);
}
