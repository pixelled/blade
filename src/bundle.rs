use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use super::LYON_SCALE;
use crate::component::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    health: Health,

    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
}

impl PlayerBundle {
    pub fn new(x: f32, y: f32) -> Self {
        PlayerBundle {
            player: Player {},
            health: Health { hp: 100 },
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 2.0),
                    scale: Vec3::new(40.0, 40.0, 0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.7, 0.7, 0.7),
                    ..Default::default()
                },
                ..Default::default()
            },
            rigid_body: RigidBodyBundle {
                position: Vec2::new(x, y).into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(2.0, 2.0).into(),
                // mass_properties: ColliderMassProps::Density(1.0).into(),
                flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
                ..Default::default()
            },
            sync: RigidBodyPositionSync::Discrete,
        }
    }
}

#[derive(Bundle)]
pub struct ObjectBundle {
    object: Object,
    throwable: Throwable,

    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
}

impl ObjectBundle {
    pub fn new(x: f32, y: f32) -> Self {
        // let shape = shapes::RegularPolygon {
        //     sides: 4,
        //     feature: shapes::RegularPolygonFeature::Radius(30.0),
        //     ..shapes::RegularPolygon::default()
        // };

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
                position: (Vec2::new(x, y), 0.0).into(),
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
}

#[derive(Bundle)]
pub struct StaticBundle {
    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
}

impl StaticBundle {
    // pub fn new(shape: &impl Geometry) -> Self {
    //
    // }

    pub fn new_rect(half_extents: Vec2, origin: Vec2) -> Self {
        let shape = shapes::Rectangle {
            extents: half_extents.clone() * 2.0 * LYON_SCALE,
            origin: RectangleOrigin::Center
        };
        StaticBundle {
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::ALICE_BLUE),
                    outline_mode: StrokeMode::new(Color::GRAY, 5.0),
                },
                Transform {
                    translation: Vec3::new(origin.x, origin.y, 1.0),
                    ..Default::default()
                },
            ),
            rigid_body: RigidBodyBundle {
                position: (origin, 0.0).into(),
                body_type: RigidBodyType::Static.into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(half_extents.x, half_extents.y).into(),
                ..Default::default()
            },
            sync: RigidBodyPositionSync::Discrete,
        }
    }
}

#[derive(Bundle)]
pub struct BarBundle {
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
}

impl BarBundle {
    pub fn new(x: f32, y: f32, asset_server: &Res<AssetServer>) -> Self {
        BarBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 1.0),
                    ..Default::default()
                },
                texture: asset_server.load("test-bar.png"),
                ..Default::default()
            },
            rigid_body: RigidBodyBundle {
                position: (Vec2::new(0.0, -5.0), 0.0).into(),
                body_type: RigidBodyType::Static.into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(20.0, 1.0).into(),
                ..Default::default()
            },
            sync: RigidBodyPositionSync::Discrete,
        }
    }
}

#[derive(Bundle)]
pub struct HealthTextBundle {
    #[bundle]
    text: TextBundle
}

impl HealthTextBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        HealthTextBundle {
            text: TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Health: ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.5, 0.5, 1.0),
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(1.0, 0.5, 0.5),
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}

#[derive(Component)]
pub struct HealthBarDisplay;

#[derive(Component)]
pub struct HealthBarDisplayComponent {
    pub cur_percent: f32,
    displayed_percent: f32,
}

impl HealthBarDisplayComponent {
    pub fn animate(&mut self, mut transform: Mut<Transform>) {
        if self.displayed_percent != self.cur_percent {
            self.displayed_percent = self.cur_percent.max(self.displayed_percent - 0.9);
        }
        let new_x = 350.0 * self.displayed_percent / 100.0;
        transform.scale = Vec3::new(new_x, 15.0, 0.0).into();
    }
}

pub fn spawn_health_bar(commands: &mut Commands) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -50.0, 2.0),
            scale: Vec3::new(350.0, 15.0, 0.0).into(),
            ..Default::default()
        },
        sprite: Sprite {
            color: Color::rgb_u8(184, 248, 174),
            ..Default::default()
        },
        ..Default::default()
    }).insert(HealthBarDisplayComponent {
        cur_percent: 1.0,
        displayed_percent: 1.0,
    });
}
