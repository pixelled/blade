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
    pub shapes: std::vec::Vec<u8>,
}

impl Storage {
    pub fn insert(&mut self, id: u8) {
        for i in self.shapes.iter_mut() {
            if *i == 0 {
                *i = id;
                break;
            }
        }
    }
}

#[derive(Component)]
pub struct EndGameUI;
