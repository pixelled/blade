use bevy::prelude::*;

use crate::AppState;
use crate::component::*;
use crate::in_game::ObjectToPlayer;
use crate::particle::ParticleEvent;

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(HealTimer)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(heal_system)
                    .with_system(sight_system)
                    .with_system(heal_animation_system)
            );
    }
}

#[derive(Component)]
pub struct Heal {
    pub hp: i32,
}

#[derive(Component)]
pub struct Sight {
    pub radius: f32
}

fn heal_system(
    object_to_player: Res<ObjectToPlayer>,
    object_query: Query<(Entity, &Heal)>,
    mut player_query: Query<&mut Health>,
) {
    for (object_entity, heal) in object_query.iter() {
        let player_entity = object_to_player.0.get(&object_entity).unwrap();
        let mut player_health = player_query.get_mut(*player_entity).unwrap();
        player_health.hp += heal.hp;
    }
}

struct HealTimer(Timer);

fn heal_animation_system(
    mut ev_particle: EventWriter<ParticleEvent>,
    time: Res<Time>,
    mut timer: ResMut<HealTimer>,
    q: Query<&Transform, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let q = q.single();
        ev_particle.send(ParticleEvent {
            pos: q.translation,
            num: 3,
            color: Color::rgb_u8(184, 248, 174)
        });
    }
}

fn sight_system() {
    todo!()
}