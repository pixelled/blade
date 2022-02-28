use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::AppState;
use crate::in_game::EntityInHand;
use crate::component::*;
use crate::shape_mod::*;
use crate::ui::*;

use bevy::utils::HashMap;
use crate::bundle::CommandsSpawner;

pub const STORAGE_SIZE: usize = 8;
pub const BLUEPRINT_SIZE: usize = 4;

pub struct SynthesisPlugin;

impl Plugin for SynthesisPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(UIPlugin)
            .init_resource::<StorageInHand>()
            .add_system_set(
                SystemSet::on_enter(AppState::Setup)
                    .with_system(setup_table)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(set_recipe_global_transform)
                    .with_system(storage_input)
                    .with_system(clear_entity)
                    .with_system(synthesize_entity)
                    .with_system(store_entity)
                    .with_system(hold_stored_entity)
                    .with_system(button_system)
            );
    }
}

pub struct Table(pub HashMap<Vec<(Type, usize)>, Type>);
pub struct TableInverse(pub HashMap<usize, Vec<(Type, usize)>>);

fn setup_table(mut commands: Commands) {
    let t = init_table();
    let m = t.clone().into_iter().collect();
    let m_inverse = t.into_iter().map(|(k, v)| (v as usize, k)).collect();
    commands.insert_resource(Table(m));
    commands.insert_resource(TableInverse(m_inverse))
}

fn storage_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut storage_in_hand: ResMut<StorageInHand>,
    mut q: Query<(&Storage, &mut Blueprint)>,
) {
    let keys = vec![
        KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
        KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8,
    ];
    if keyboard_input.any_just_pressed(keys) {
        storage_in_hand.prev = storage_in_hand.cur;
        if keyboard_input.just_pressed(KeyCode::Key1) {
            storage_in_hand.cur = Some(0);
        } else if keyboard_input.just_pressed(KeyCode::Key2) {
            storage_in_hand.cur = Some(1);
        } else if keyboard_input.just_pressed(KeyCode::Key3) {
            storage_in_hand.cur = Some(2);
        } else if keyboard_input.just_pressed(KeyCode::Key4) {
            storage_in_hand.cur = Some(3);
        } else if keyboard_input.just_pressed(KeyCode::Key5) {
            storage_in_hand.cur = Some(4);
        } else if keyboard_input.just_pressed(KeyCode::Key6) {
            storage_in_hand.cur = Some(5);
        } else if keyboard_input.just_pressed(KeyCode::Key7) {
            storage_in_hand.cur = Some(6);
        } else if keyboard_input.just_pressed(KeyCode::Key8) {
            storage_in_hand.cur = Some(7);
        }
        // add to blueprint if double clicked
        if storage_in_hand.prev == storage_in_hand.cur {
            if let Some(idx) = storage_in_hand.prev {
                let (s, mut bq): (&Storage, Mut<Blueprint>) = q.single_mut();
                bq.insert(s.items[idx]);
            }
        }
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
    query_id: Query<&Throwable>,
) {
    if keyboard_input.just_pressed(KeyCode::F) {
        if let Some(e_in_hand) = entity_in_hand.entity {
            let mut player_query = q.q0();
            let (player_entity, mut storage): (Entity, Mut<Storage>) = player_query.single_mut();
            if !storage.insert(query_id.get(e_in_hand).unwrap().0) {
                return
            }
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

fn hold_stored_entity(
    mut commands: Commands,
    windows: Res<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    storage_in_hand: Res<StorageInHand>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: Query<(Entity, &mut Storage, &RigidBodyPositionComponent), With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::E) {
        return;
    }
    let (player_entity, mut storage, rb_pos) = q.single_mut();
    if let Some(i) = storage_in_hand.cur {
        let id = storage.items[i];
        if id != Type::Empty {
            let window = windows.get_primary().unwrap();
            use nalgebra::UnitComplex;
            if let Some(pos) = window.cursor_position() {
                let size = Vec2::new(window.width() as f32, window.height() as f32);
                let pos = pos - size / 2.0;
                let cursor_rot = UnitComplex::new(pos.y.atan2(pos.x));
                let object_entity = commands.spawn_object(
                    id,
                    [rb_pos.position.translation.x + 7.0 * cursor_rot.cos_angle(),
                         rb_pos.position.translation.y + 7.0 * cursor_rot.sin_angle()]
                ).id();
                let axis = Vector::x_axis();
                let joint = PrismaticJoint::new(axis)
                    .local_anchor1(point![0.0, 0.0])
                    .local_anchor2(point![0.0, 0.0])
                    .limit_axis([6.5, 8.0]);
                commands
                    .spawn()
                    .insert(JointBuilderComponent::new(joint, player_entity, object_entity));
                entity_in_hand.entity = Some(object_entity);
                commands.entity(object_entity).insert(Grabbed(player_entity));
                storage.items[i] = Type::Empty;
            }
        }
    }
}

fn check_ingredients(sto: &Storage, bp: &Blueprint) -> (bool, Vec<usize>) {
    let mut bp_map: HashMap<Type, usize> = bp.clone().into();
    let mut indices = vec![];
    for (idx, id) in sto.items.iter().enumerate() {
        match bp_map.get_mut(id) {
            Some(c) => {
                *c -= 1;
                indices.push(idx);
                if *c == 0 {
                    bp_map.remove(id);
                }
            },
            None => {}
        }
    }
    if bp_map.is_empty() {
        (true, indices)
    } else {
        (false, vec![])
    }
}

fn synthesize_entity(
    keyboard_input: Res<Input<KeyCode>>,
    table: Res<Table>,
    mut q: Query<(&mut Storage, &Blueprint)>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        let (mut storage, bp): (Mut<Storage>, &Blueprint) = q.single_mut();
        let bp_vec: std::vec::Vec<(Type, usize)> = bp.clone().into();
        match table.0.get(&bp_vec) {
            Some(&id) => {
                let (is_enough, indices) = check_ingredients(&storage, &bp);
                if is_enough {
                    storage.remove(&indices);
                    storage.insert(id);
                }
            },
            None => {}
        }
    }
}

fn clear_entity(
    keyboard_input: Res<Input<KeyCode>>,
    mut bp_query: Query<&mut Blueprint>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        let mut bp = bp_query.single_mut();
        bp.clear();
    }
}
