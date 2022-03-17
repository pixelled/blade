use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use super::RAPIER_TO_LYON;
use crate::component::*;
use crate::magic::*;
use crate::shape_mod::*;
use crate::synthesis::*;
use crate::SpriteAtlasHandle;
use bevy::ecs::system::EntityCommands;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    health: Health,
    dmg: Dmg,
    storage: Storage,
    blueprint: Blueprint,

    #[bundle]
    sprite: SpriteSheetBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
}

#[derive(Bundle)]
pub struct ObjectBundle {
    pub object: Object,
    pub throwable: Throwable,

    pub health: Health,
    pub dmg: Dmg,

    #[bundle]
    pub sprite: SpriteSheetBundle,
    #[bundle]
    pub rigid_body: RigidBodyBundle,
    #[bundle]
    pub collider: ColliderBundle,
    pub sync: RigidBodyPositionSync,
}

impl Default for ObjectBundle {
    fn default() -> Self {
        ObjectBundle {
            object: Object {},
            throwable: Throwable(Type::Empty),
            health: Health { hp: 2 },
            dmg: Dmg(1),
            sprite: SpriteSheetBundle::default(),
            rigid_body: RigidBodyBundle {
                ..Default::default()
            },
            collider: ColliderBundle {
                ..Default::default()
            },
            sync: RigidBodyPositionSync::Discrete,
        }
    }
}

pub trait CommandsSpawner<'w, 's> {
    fn spawn_player<'a>(
        &'a mut self,
        sprite_atlas_handle: &SpriteAtlasHandle,
        x: f32,
        y: f32,
    ) -> EntityCommands<'w, 's, 'a>;

    fn spawn_sprite<'a>(
        &'a mut self,
        sprite_atlas_handle: &SpriteAtlasHandle,
        id: Type,
        pos: [f32; 2],
    ) -> EntityCommands<'w, 's, 'a>;

    fn spawn_object<'a>(
        &'a mut self,
        sprite_atlas_handle: &SpriteAtlasHandle,
        id: Type,
        pos: [f32; 2],
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> CommandsSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_player<'a>(
        &'a mut self,
        sprite_atlas_handle: &SpriteAtlasHandle,
        x: f32,
        y: f32,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut e = self.spawn();
        e.insert_bundle(PlayerBundle {
            player: Player {},
            health: Health { hp: 100 },
            dmg: Dmg(1),
            storage: Storage {
                items: vec![Type::Empty; STORAGE_SIZE],
            },
            blueprint: Blueprint {
                items: vec![Type::Empty; BLUEPRINT_SIZE],
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 5.0),
                    scale: Vec3::new(0.8, 0.8, 1.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(
                    sprite_atlas_handle.map[&"sprites/player/body-line.png".into()],
                ),
                texture_atlas: sprite_atlas_handle.handle.clone(),
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
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -4.0, 4.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(
                    sprite_atlas_handle.map[&"sprites/player/body-shadow.png".into()],
                ),
                texture_atlas: sprite_atlas_handle.handle.clone(),
                ..Default::default()
            });
        });
        e
    }

    fn spawn_sprite<'a>(
        &'a mut self,
        sprite_atlas_handle: &SpriteAtlasHandle,
        id: Type,
        pos: [f32; 2],
    ) -> EntityCommands<'w, 's, 'a> {
        let mut e = self.spawn();
        // e
        //     .insert_bundle(OUTLINES[id as usize](&asset_server, Usage::Storage))
        //     .with_children(|parent| {
        //         // parent.spawn_bundle(SpriteBundle {
        //         //    // TODO: shadow
        //         //     transform: Transform::from_translation(Vec3::new(0.0, -4.0, 4.0)),
        //         //     texture: asset_server.get_handle("circle/shadow.png"),
        //         //     ..Default::default()
        //         // });
        //         // parent.spawn_bundle(SpriteBundle {
        //         //    // TODO: color
        //         //     transform: Transform {
        //         //         translation: Vec3::new(0.0, 0.0, 3.0),
        //         //         ..Default::default()
        //         //     },
        //         //     texture: asset_server.get_handle("circle/color.png"),
        //         //     ..Default::default()
        //         // });
        //     });
        e
    }

    fn spawn_object<'a>(
        &'a mut self,
        sprite_atlas_handle: &SpriteAtlasHandle,
        id: Type,
        pos: [f32; 2],
    ) -> EntityCommands<'w, 's, 'a> {
        let mut e = self.spawn();
        e.insert_bundle(OBJECTS[id as usize](sprite_atlas_handle, Vec2::from(pos)))
            .with_children(|parent| {
                parent.spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, -4.0, 6.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(
                        sprite_atlas_handle.map[&"sprites/circle/shadow.png".into()],
                    ),
                    texture_atlas: sprite_atlas_handle.handle.clone(),
                    ..Default::default()
                });
            });
        match id {
            Type::Heart => {
                e.insert(Heal::new(1, 0.1))
                    .insert(FreezeSource::new(0.97, 2.0));
            }
            Type::Square => {
                e.insert(ParalyzeSource::new(1.0));
            }
            Type::Circle => {
                e.insert(Sight::new(1.5)).insert(Explode::new(20.0, 20));
            }
            Type::Triangle => {
                e.insert(BurnSource::new(1, 5.0, 0.5));
            }
            _ => {}
        }
        e
    }
}

/// object!(id: Type, pos: [f32, f32])
#[macro_export]
macro_rules! object {
    ($id:expr, $pos:expr) => {
        OBJECTS[$id as usize](Vec2::from($pos))
    };
}

#[derive(Component)]
pub struct Undead;

#[derive(Bundle)]
pub struct StaticBundle {
    health: Health,
    dmg: Dmg,

    #[bundle]
    shape: ShapeBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
    undead: Undead,
}

impl StaticBundle {
    pub fn new_rect(half_extents: Vec2, origin: Vec2) -> Self {
        let shape = shapes::Rectangle {
            extents: half_extents.clone() * 2.0 * RAPIER_TO_LYON,
            origin: RectangleOrigin::Center,
        };
        StaticBundle {
            health: Health { hp: 0 },
            dmg: Dmg(1),
            shape: GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::ALICE_BLUE),
                    outline_mode: StrokeMode::new(Color::rgba(0.0, 0.0, 0.0, 0.0), 5.0),
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
            undead: Undead {},
        }
    }
}

#[derive(Bundle)]
pub struct BarBundle {
    health: Health,
    dmg: Dmg,

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
            health: Health { hp: 0 },
            dmg: Dmg(1),
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
