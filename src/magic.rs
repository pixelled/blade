use bevy::prelude::*;

use crate::AppState;
use crate::component::*;
use crate::camera::*;
use crate::particle::DespawnParticles;

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(heal_timer_system)
                    .with_system(heal_system)
                    .with_system(sight_system)
                    .with_system(heal_animation_system)
            );
    }
}

#[derive(Component)]
pub struct Heal {
    pub hp: i32,
    pub timer: Timer,
}

impl Default for Heal {
    fn default() -> Self {
        Heal {
            hp: 1,
            timer: Timer::from_seconds(0.1, true)
        }
    }
}

#[derive(Component)]
pub struct Sight {
    pub radius: f32
}

fn heal_timer_system(
    time: Res<Time>,
    mut magic_query: Query<&mut Heal, With<Grabbed>>,
) {
    for mut heal in magic_query.iter_mut() {
        heal.timer.tick(time.delta());
    }
}

fn heal_system(
    object_query: Query<(&Heal, &Grabbed)>,
    mut player_query: Query<&mut Health>,
) {
    for (heal, grabbed) in object_query.iter() {
        if heal.timer.just_finished() {
            let player_entity = grabbed.0;
            let mut player_health = player_query.get_mut(player_entity).unwrap();
            player_health.heal(heal.hp);
        }
    }
}

fn heal_animation_system(
    mut ev_particle: EventWriter<DespawnParticles>,
    object_query: Query<(&Heal, &Grabbed)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (heal, grabbed) in object_query.iter() {
        if heal.timer.just_finished() {
            let player_entity = grabbed.0;
            let player_pos = player_query.get(player_entity).unwrap();
            ev_particle.send(DespawnParticles {
                pos: Vec3::from([player_pos.translation.x, player_pos.translation.y, 20.0]),
                num: 3,
                color: Color::rgb_u8(184, 248, 174)
            });
        }
    }
}

fn sight_system(
    mut camera: Query<(&mut OrthographicProjection, &MainCamera)>,
    object_query: Query<(&Sight, &Grabbed)>,
) {
    let (mut camera_config, camera) = camera.single_mut();
    if object_query.is_empty() {
        camera_config.scale = (camera_config.scale - camera.speed_z).max(1.0);
    } else {
        camera_config.scale = (camera_config.scale + camera.speed_z).min(1.5);
    }
}