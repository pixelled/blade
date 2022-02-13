use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::AppState;
use crate::in_game::EntityInHand;
use crate::component::*;
use crate::bundle::*;
use crate::shape_mod::*;
use crate::ui::*;
use crate::camera::MainCamera;

use std::collections::HashMap;
use bevy_prototype_lyon::prelude::GeometryBuilder;
use bevy_prototype_lyon::entity::ShapeBundle;

pub const STORAGE_SIZE: usize = 4;
pub const BLUEPRINT_SIZE: usize = 4;

pub struct SynthesisPlugin;

impl Plugin for SynthesisPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StorageUIs>()
            .init_resource::<BlueprintUIs>()
            .init_resource::<StorageInHand>()
            .add_system_set(
                SystemSet::on_enter(AppState::Setup)
                    .with_system(setup_table)
                    .with_system(setup_storage_display)
                    .with_system(setup_blueprint_display)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(storage_input)
                    .with_system(clear_entity)
                    .with_system(synthesize_entity)
                    .with_system(store_entity)
                    .with_system(hold_stored_entity)
                    .with_system(set_storage_global_transform)
                    .with_system(set_blueprint_global_transform)
                    .with_system(update_storage_display)
                    .with_system(update_blueprint_display)
            );
    }
}

pub struct Table(pub HashMap<String, u8>);

fn setup_table(mut commands: Commands) {
    let t = init_table();
    let m = t.into_iter().map(|(e, v)| unsafe {
        (String::from_utf8_unchecked(e), v)
    }).collect();
    commands.insert_resource(Table(m));
}

fn storage_input(
    keybord_input: Res<Input<KeyCode>>,
    mut storage_in_hand: ResMut<StorageInHand>,
    mut q: Query<(&Storage, &mut Blueprint)>,
) {
    let keys = vec![KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4];
    if keybord_input.any_just_pressed(keys) {
        storage_in_hand.prev = storage_in_hand.cur;
        if keybord_input.just_pressed(KeyCode::Key1) {
            storage_in_hand.cur = Some(0);
        } else if keybord_input.just_pressed(KeyCode::Key2) {
            storage_in_hand.cur = Some(1);
        } else if keybord_input.just_pressed(KeyCode::Key3) {
            storage_in_hand.cur = Some(2);
        } else if keybord_input.just_pressed(KeyCode::Key4) {
            storage_in_hand.cur = Some(3);
        }
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
            storage.insert(query_id.get(e_in_hand).unwrap().0);
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
    keyboard_input: Res<Input<KeyCode>>,
    mut storage_in_hand: Res<StorageInHand>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: Query<(Entity, &mut Storage, &RigidBodyPositionComponent), With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::E) {
        return;
    }
    let (player_e, mut storage, rb_pos) = q.single_mut();
    if let Some(i) = storage_in_hand.cur {
        let id = storage.items[i];
        if id != 0 {
            let object = commands.spawn_bundle(
                OBJECTS[(id - 1) as usize]
                    (Vec2::new(rb_pos.position.translation.x + 10.0, rb_pos.position.translation.y))
            ).id();
            let axis = Vector::x_axis();
            let joint = PrismaticJoint::new(axis)
                .local_anchor1(point![0.0, 0.0])
                .local_anchor2(point![0.0, 0.0])
                .limit_axis([7.0, 8.0]);
            commands
                .spawn()
                .insert(JointBuilderComponent::new(joint, player_e, object));
            entity_in_hand.entity = Some(object);
            storage.items[i] = 0;
        }
    }
}

fn synthesize_entity(
    keybord_input: Res<Input<KeyCode>>,
    table: Res<Table>,
    mut q: Query<(&mut Storage, &mut Blueprint)>,
) {
    if keybord_input.just_pressed(KeyCode::Q) {
        let (mut storage, mut bp): (Mut<Storage>, _) = q.single_mut();
        let mut sorted_bp = bp.items.clone();
        sorted_bp.sort();
        println!("{:?}", sorted_bp);
        let s = unsafe { String::from_utf8_unchecked(sorted_bp) };
        match table.0.get(&s) {
            Some(&v) => {
                storage.insert(v);
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

fn setup_storage_display(
    mut commands: Commands,
    mut storage_ui: ResMut<StorageUIs>,
) {
    let storage_ui = storage_ui.as_mut();
    commands
        .spawn_bundle((Transform {
            translation: Vec3::new(0.0, 0.0, 3.0),
            ..Default::default()
        }, GlobalTransform::default()))
        .insert(StorageBox)
        .with_children(|parent| {
            let extents = Vec2::new(40.0, 40.0);
            let interval = 20.0;
            let mut cur_x = (STORAGE_SIZE - 1) as f32 * (-interval - extents.x) / 2.0;
            for i in 0..STORAGE_SIZE {
                let e = parent
                    .spawn_bundle(init_box(extents, Vec2::new(cur_x, 0.0)))
                    .insert(StorageUI { child: 0 })
                    .with_children(|parent| {

                    })
                    .id();
                storage_ui.entities.push(e);
                cur_x += extents.x + interval;
            }
        });
}

fn update_storage_display(
    mut commands: Commands,
    storage_uis: Res<StorageUIs>,
    storage_in_hand: ResMut<StorageInHand>,
    storage: Query<&Storage>,
    mut q: Query<(&mut Children, &mut StorageUI)>,
    mut transform_query: Query<&mut Transform, With<StorageShape>>
) {
    let storage = storage.single();
    // println!("{:?}", storage_in_hand);
    if storage_in_hand.cur != storage_in_hand.prev {
        if let Some(i) = storage_in_hand.prev {
            let (mut children, _) = q.get_mut(storage_uis.entities[i]).unwrap();
            for child in children.iter() {
                if let Ok(mut transform) = transform_query.get_mut(*child) {
                    transform.rotation = Quat::from_rotation_y(0.0);
                }
            }
        }
    }
    if let Some(i) = storage_in_hand.cur {
        let (mut children, _) = q.get_mut(storage_uis.entities[i]).unwrap();
        for child in children.iter() {
            if let Ok(mut transform) = transform_query.get_mut(*child) {
                transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_y(0.03));
            }
        }
    }
    for (i, &id) in storage.items.iter().enumerate() {
        let parent = storage_uis.entities[i];
        let (mut children, mut storage_ui): (_, Mut<StorageUI>) = q.get_mut(parent).unwrap();
        if id != storage_ui.child {
            if id == 0 {
                commands.entity(parent).despawn_descendants();
            } else {
                let (f, scale) = SHAPES[(id - 1) as usize];
                let child = commands
                    .spawn_bundle(f(scale))
                    .insert(StorageShape {})
                    .id();
                commands.entity(parent).push_children(&[child]);
            }
            storage_ui.child = id;
        }
    }
}

fn set_storage_global_transform(
    wnds: Res<Windows>,
    mut q: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<&mut Transform, With<StorageBox>>
    )>
) {
    let wnd = wnds.get_primary().unwrap();
    let camera_pos = q.q0().single();
    let x = camera_pos.translation.x;
    let y = camera_pos.translation.y - wnd.height() as f32 / 2.3;

    let mut q1 = q.q1();
    let mut storage_pos = q1.single_mut();
    storage_pos.translation.x = x;
    storage_pos.translation.y = y;
}

fn setup_blueprint_display(
    mut commands: Commands,
    mut blueprint_uis: ResMut<BlueprintUIs>,
) {
    let blueprint_uis = blueprint_uis.as_mut();
    let parent = commands.spawn_bundle(
        (Transform {
            translation: Vec3::new(0.0, 0.0, 3.0),
            ..Default::default()
        }, GlobalTransform::default())
    ).insert(BlueprintBox)
        .with_children(|parent| {
            let extents = Vec2::new(40.0, 40.0);
            let interval = 0.0;
            let mut cur_x = (BLUEPRINT_SIZE - 1) as f32 * (-interval - extents.x) / 2.0;
            for i in 0..BLUEPRINT_SIZE {
                let e = parent
                    .spawn_bundle(init_box(extents, Vec2::new(cur_x, 0.0)))
                    .insert(BlueprintUI { child: 0 })
                    .with_children(|parent| {
                    })
                    .id();
                blueprint_uis.entities.push(e);
                cur_x += extents.x + interval;
            }
        });
}

fn update_blueprint_display(
    mut commands: Commands,
    blueprint_uis: Res<BlueprintUIs>,
    blueprint: Query<&Blueprint>,
    mut q: Query<(&mut Children, &mut BlueprintUI)>,
    mut transform_query: Query<&mut Transform, With<BlueprintShape>>
) {
    let bp = blueprint.single();
    for (i, &id) in bp.items.iter().enumerate() {
        let parent = blueprint_uis.entities[i];
        let (mut children, mut blueprint_ui): (_, Mut<BlueprintUI>) = q.get_mut(parent).unwrap();
        if id != blueprint_ui.child {
            if id == 0 {
                commands.entity(parent).despawn_descendants();
            } else {
                let (f, scale) = SHAPES[(id - 1) as usize];
                let child = commands
                    .spawn_bundle(f(scale))
                    .insert(BlueprintShape {})
                    .id();
                commands.entity(parent).push_children(&[child]);
            }
            blueprint_ui.child = id;
        }
    }
}

fn set_blueprint_global_transform(
    wnds: Res<Windows>,
    mut q: QuerySet<(
        QueryState<&Transform, With<MainCamera>>,
        QueryState<&mut Transform, With<BlueprintBox>>
    )>
) {
    let wnd = wnds.get_primary().unwrap();
    let camera_pos = q.q0().single();
    let x = camera_pos.translation.x - wnd.width() as f32 / 2.5;
    let y = camera_pos.translation.y - wnd.height() as f32 / 2.3;

    let mut q1 = q.q1();
    let mut blueprint_pos = q1.single_mut();
    blueprint_pos.translation.x = x;
    blueprint_pos.translation.y = y;
}
