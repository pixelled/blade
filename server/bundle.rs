use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::component::*;
use crate::magic::*;
use crate::shape_mod::*;
use crate::synthesis::*;
use bevy::ecs::system::EntityCommands;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    health: Health,
    dmg: Dmg,
    storage: Storage,
    blueprint: Blueprint,

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
        x: f32,
        y: f32,
    ) -> EntityCommands<'w, 's, 'a>;

    fn spawn_object<'a>(
        &'a mut self,
        id: Type,
        pos: [f32; 2],
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> CommandsSpawner<'w, 's> for Commands<'w, 's> {
    fn spawn_player<'a>(
        &'a mut self,
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
        });
        e
    }

    fn spawn_object<'a>(
        &'a mut self,
        id: Type,
        pos: [f32; 2],
    ) -> EntityCommands<'w, 's, 'a> {
        let mut e = self.spawn();
        e.insert_bundle(OBJECTS[id as usize](Vec2::from(pos)));
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
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    sync: RigidBodyPositionSync,
    undead: Undead,
}

impl StaticBundle {
    pub fn new_rect(half_extents: Vec2, origin: Vec2) -> Self {
        StaticBundle {
            health: Health { hp: 0 },
            dmg: Dmg(1),
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
