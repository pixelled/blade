use bevy::prelude::*;
use crate::shape_mod::Type;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Throwable(pub Type);

#[derive(Component)]
pub struct Health {
    pub hp: i32,
}

#[derive(Component)]
pub struct Damage(i32);

#[derive(Component)]
pub struct Storage {
    pub items: std::vec::Vec<Type>,
}

impl Storage {
    pub fn insert(&mut self, id: Type) -> bool {
        for i in self.items.iter_mut() {
            if *i == Type::Empty {
                *i = id;
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, indices: &[usize]) {
        indices.iter().for_each(|idx| {
            self.items[*idx] = Type::Empty;
        });
    }
}

impl From<Storage> for HashMap<Type, usize> {
    fn from(sto: Storage) -> Self {
        sto.items.into_iter().filter(|x| *x != Type::Empty).counts()
    }
}

#[derive(Component, Clone)]
pub struct Blueprint {
    pub items: std::vec::Vec<Type>,
}

impl From<Blueprint> for std::vec::Vec<(Type, usize)> {
    fn from(mut bp: Blueprint) -> Self {
        bp.items.sort_by(|a, b| b.cmp(a));
        let m = bp.items.into_iter().filter(|x| *x != Type::Empty).counts();
        m.into_iter().collect()
    }
}

impl From<Blueprint> for HashMap<Type, usize> {
    fn from(bp: Blueprint) -> Self {
        bp.items.into_iter().filter(|x| *x != Type::Empty).counts()
    }
}

impl Blueprint {
    pub fn insert(&mut self, id: Type) {
        for i in self.items.iter_mut() {
            if *i == Type::Empty {
                *i = id;
                break;
            }
        }
    }

    pub fn clear(&mut self) {
        self.items.iter_mut().for_each(|v| { *v = Type::Empty });
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
