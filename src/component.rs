use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Health {
    pub hp: i32,
}

#[derive(Component)]
pub struct EndGameUI;
