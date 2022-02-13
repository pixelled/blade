use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Throwable(pub u8);

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

    pub fn clear(&mut self) {
        self.items.iter_mut().for_each(|v| { *v = 0 });
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct StorageInHand {
    pub prev: Option<usize>,
    pub cur: Option<usize>
}

#[derive(Component)]
pub struct EndGameUI;
