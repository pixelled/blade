use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::physics::RigidBodyComponentsQueryPayload;

use super::AppState;
use crate::in_game::EntityInHand;
use crate::component::*;
use crate::bundle::*;

pub const STORAGE_SIZE: usize = 4;

pub struct SynthesisPlugin;

impl Plugin for SynthesisPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StorageUIs>()
            .init_resource::<StorageInHand>()
            .add_system_set(
                SystemSet::on_enter(AppState::Setup)
                    .with_system(setup_storage_display)
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(storage_input)
                    .with_system(store_entity)
                    .with_system(hold_stored_entity)
                    .with_system(storage_display)
            );
    }
}

fn storage_input(
    keybord_input: Res<Input<KeyCode>>,
    mut storage_in_hand: ResMut<StorageInHand>,
) {
    // let storage = storage.single();
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
            storage_in_hand.cur = None;
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

) {

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
        if storage.shapes[i] != 0 {
            let object = commands.spawn_bundle(ObjectBundle::new(
                Vec2::new(rb_pos.position.translation.x + 10.0, rb_pos.position.translation.y), i
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
            storage.shapes[i] = 0;
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

fn storage_display(
    asset_server: ResMut<AssetServer>,
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
    for (i, id) in storage.shapes.iter().enumerate() {
        let (mut ui_image, mut transform, mut visibility, mut storage_ui) = q.get_mut(storage_uis.uis[i]).unwrap();
        if *id == 0 {
            visibility.is_visible = false;
        } else {
            visibility.is_visible = true;
        }
    }
}
