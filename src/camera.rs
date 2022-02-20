use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::component::Player;
use crate::RAPIER_TO_BEVY;

#[derive(Component)]
pub struct MainCamera {
    // speed: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            // speed: 0.5,
        }
    }
}

pub fn move_camera(
    player: Query<&RigidBodyPositionComponent, With<Player>>,
    mut camera: Query<(&mut Transform, &MainCamera)>
) {
    let position = player.single();
    let player_translation = &position.position.translation;
    let (mut camera_transform, _main_camera): (Mut<Transform>, &MainCamera) = camera.single_mut();
    let camera_translation = camera_transform.translation;
    let dir = Vec2::new(
        player_translation.x * RAPIER_TO_BEVY - camera_translation.x,
        player_translation.y * RAPIER_TO_BEVY - camera_translation.y
    );
    camera_transform.translation.x += dir.x * 0.5;
    camera_transform.translation.y += dir.y * 0.5;
    // camera_transform.translation.x = player_translation.x * RAPIER_SCALE;
    // camera_transform.translation.y = player_translation.y * RAPIER_SCALE;
}
