use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::AppState;
use crate::in_game::EntityInHand;
use crate::component::*;
use crate::bundle::*;
use crate::shape_mod::*;

use std::collections::HashMap;

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
                    .with_system(setup_table_display)
                    .with_system(setup_storage_display)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(storage_input)
                    .with_system(synthesize_entity)
                    .with_system(store_entity)
                    .with_system(hold_stored_entity)
                    .with_system(storage_display)
                    .with_system(blueprint_display)
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

fn hold_stored_entity(
    mut commands: Commands,
    mut keyboard_input: Res<Input<KeyCode>>,
    mut storage_in_hand: Res<StorageInHand>,
    mut entity_in_hand: ResMut<EntityInHand>,
    mut q: Query<(Entity, &mut Storage, &RigidBodyPositionComponent), With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::E) {
        return;
    }
    let (player_e, mut storage, rb_pos) = q.single_mut();
    if let Some(i) = storage_in_hand.cur {
        if storage.items[i] != 0 {
            let object = commands.spawn_bundle(ObjectBundle::new(
                Vec2::new(rb_pos.position.translation.x + 10.0, rb_pos.position.translation.y), 1
            )).id();
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

fn setup_storage_display(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut storage_ui: ResMut<StorageUIs>,
) {
    let storage_ui = storage_ui.as_mut();
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
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: Color::rgba(0.9, 0.9, 0.9, 0.7).into(),
                        ..Default::default()
                    }).with_children(|parent| {
                        let e = parent.spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Percent(60.0), Val::Percent(60.0)),
                                ..Default::default()
                            },
                            image: asset_server.load("yellow-sq.png").into(),
                            visibility: Visibility { is_visible: false},
                            ..Default::default()
                        }).insert(StorageUI {}).id();
                        storage_ui.uis.push(e);
                    });
                });
            }
        });
}

fn setup_table_display(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut blueprint_uis: ResMut<BlueprintUIs>,
) {
    let blueprint_uis = blueprint_uis.as_mut();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(15.0)),
                // padding: Rect::all(Val::Percent(25.0)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0.8, 0.0, 1.0, 1.0).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            for _ in 0..BLUEPRINT_SIZE {
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
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        color: Color::rgba(0.9, 0.9, 0.9, 0.7).into(),
                        ..Default::default()
                    }).with_children(|parent| {
                        let e = parent.spawn_bundle(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Percent(60.0), Val::Percent(60.0)),
                                ..Default::default()
                            },
                            image: asset_server.load("yellow-sq.png").into(),
                            visibility: Visibility { is_visible: false},
                            ..Default::default()
                        }).insert(BlueprintUI {}).id();
                        blueprint_uis.0.push(e);
                    });
                });
            }
        });
}

fn storage_display(
    storage_uis: Res<StorageUIs>,
    storage_in_hand: ResMut<StorageInHand>,
    storage: Query<&Storage>,
    mut q: Query<(&mut UiImage, &mut Transform, &mut Visibility, &mut StorageUI)>
) {
    let storage = storage.single();
    if storage_in_hand.cur != storage_in_hand.prev {
        if let Some(i) = storage_in_hand.prev {
            let (_, mut transform, _, _) = q.get_mut(storage_uis.uis[i]).unwrap();
            transform.rotation = Quat::from_rotation_z(0.0);
        }
    }
    if let Some(i) = storage_in_hand.cur {
        let (_, mut transform, _, _) = q.get_mut(storage_uis.uis[i]).unwrap();
        transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_z(0.03));
    }
    for (i, id) in storage.items.iter().enumerate() {
        let (mut ui_image, mut transform, mut visibility, mut storage_ui) = q.get_mut(storage_uis.uis[i]).unwrap();
        if *id == 0 {
            visibility.is_visible = false;
        } else {
            visibility.is_visible = true;
        }
    }
}

fn blueprint_display(
    blueprint_uis: Res<BlueprintUIs>,
    blueprint: Query<&Blueprint>,
    mut q: Query<(&mut UiImage, &mut Transform, &mut Visibility, &mut BlueprintUI)>
) {
    let bp = blueprint.single();
    for (i, id) in bp.items.iter().enumerate() {
        let (mut ui_image, mut transform, mut visibility, mut storage_ui) = q.get_mut(blueprint_uis.0[i]).unwrap();
        if *id == 0 {
            visibility.is_visible = false;
        } else {
            visibility.is_visible = true;
        }
    }
}
