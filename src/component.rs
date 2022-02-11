use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Throwable;

#[derive(Component)]
pub struct Health {
    pub hp: i32,
}

#[derive(Component)]
pub struct Damage(i32);

#[derive(Component)]
pub struct Storage {
    pub items: std::vec::Vec<u8>,
}

impl Storage {
    pub fn insert(&mut self, id: u8) {
        for i in self.items.iter_mut() {
            if *i == 0 {
                *i = id;
                break;
            }
        }
    }
}

#[derive(Component)]
pub struct Blueprint {
    pub items: std::vec::Vec<u8>,
}

impl Blueprint {
    pub fn insert(&mut self, id: u8) {
        for i in self.items.iter_mut() {
            if *i == 0 {
                *i = id;
                break;
            }
        }
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct StorageInHand {
    pub prev: Option<usize>,
    pub cur: Option<usize>
}

#[derive(Default)]
pub struct StorageUIs {
    pub uis: std::vec::Vec<Entity>,
}

#[derive(Default)]
pub struct BlueprintUIs(pub std::vec::Vec<Entity>);

#[derive(Component)]
pub struct StorageUI {
    // pub rot: f32,
}

#[derive(Component)]
pub struct BlueprintUI;

#[derive(Component)]
pub struct EndGameUI;
