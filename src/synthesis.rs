use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::AppState;
use crate::in_game::EntityInHand;
use crate::component::*;

pub const STORAGE_SIZE: usize = 4;

pub struct SynthesisPlugin;

impl Plugin for SynthesisPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Setup)
                    .with_system(setup_storage_display)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(store_entity)
                    .with_system(storage_display)
            );
    }
}

fn store_entity(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut joint_set: ResMut<ImpulseJointSet>,
    mut island_manager: ResMut<IslandManager>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: QuerySet<(
        QueryState<(Entity, &mut Storage), With<Player>>,
        QueryState<RigidBodyComponentsQueryPayload>
    )>,
) {
    if keyboard_input.just_pressed(KeyCode::F) {
        if let Some(e_in_hand) = entity_in_hand.entity {
            let mut player_query = q.q0();
            let (player_entity, mut storage): (Entity, Mut<Storage>) = player_query.single_mut();
            storage.insert(1);
            let rigid_body_handle: RigidBodyHandle = player_entity.handle();
            let mut rigid_body_set = RigidBodyComponentsSet(q.q1());
            joint_set.remove_joints_attached_to_rigid_body(
                rigid_body_handle,
                &mut island_manager,
                &mut rigid_body_set,
            );
            commands.entity(e_in_hand).despawn();
            entity_in_hand.entity = None;
        }
    }
}

fn synthesize_entity() {

}

fn hold_stored_entity() {

}

fn setup_storage_display(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                // padding: Rect::all(Val::Percent(25.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0.8, 0.8, 0.8, 0.0).into(),
            ..Default::default()
        })
        .insert(EndGameUI {})
        .with_children(|parent| {
            for _ in 0..STORAGE_SIZE {
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                        margin: Rect::all(Val::Px(10.0)),
                        border: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    color: Color::rgba(0.6, 0.6, 0.6, 0.7).into(),
                    ..Default::default()
                }).with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        color: Color::rgba(0.9, 0.9, 0.9, 0.7).into(),
                        ..Default::default()
                    });
                });
            }
        });
}

fn storage_display(q: Query<&Storage>) {
    let storage = q.single();
}
