use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::component::Player;
use crate::AppState;
use crate::RAPIER_TO_BEVY;

const PLAYER_VIEW_WIDTH: f32 = 1920.0;
const PLAYER_VIEW_HEIGHT: f32 = 1080.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Setup).with_system(setup_camera))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(scale_camera))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(move_camera)
                    .label("camera")
                    .before("general"),
            );
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub speed_z: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera { speed_z: 0.01 }
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera::default());
}

pub fn move_camera(
    player: Query<&RigidBodyPositionComponent, With<Player>>,
    mut camera: Query<(&mut Transform, &MainCamera)>,
) {
    let player_pos = player.single();
    let player_translation = &player_pos.position.translation;
    let (mut camera_transform, _main_camera): (Mut<Transform>, &MainCamera) = camera.single_mut();
    let camera_translation = camera_transform.translation;
    let dir = Vec2::new(
        player_translation.x * RAPIER_TO_BEVY - camera_translation.x,
        player_translation.y * RAPIER_TO_BEVY - camera_translation.y,
    );
    camera_transform.translation.x += dir.x * 0.5;
    camera_transform.translation.y += dir.y * 0.5;
    // camera_transform.translation.x = player_pos.x * RAPIER_TO_BEVY;
    // camera_transform.translation.y = player_pos.y * RAPIER_TO_BEVY;
}

pub fn scale_camera(
    wnds: Res<Windows>,
    mut camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if wnds.is_changed() {
        let wnd = wnds.get_primary().unwrap();
        let width = wnd.width();
        let height = wnd.height();
        let width_scale = width / PLAYER_VIEW_WIDTH;
        let height_scale = height / PLAYER_VIEW_HEIGHT;
        let mut camera_config = camera.single_mut();
        let scale = if width_scale < height_scale {
            PLAYER_VIEW_HEIGHT / height
        } else {
            PLAYER_VIEW_WIDTH / width
        };
        camera_config.scale = scale;
    }
}
