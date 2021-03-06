use bevy::prelude::*;
use bevy::ui::PositionType::Absolute;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::animation::Animation;
use crate::camera::*;
use crate::component::*;
use crate::shape_mod::*;
use crate::synthesis::*;
use crate::AppState;

pub struct UIPlugin;
//
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        //         app.init_resource::<StorageUIs>()
        //             .init_resource::<BlueprintUIs>()
        //             .add_system_set(
        //                 SystemSet::on_enter(AppState::Setup)
        //                     .with_system(init_ui)
        //                     .with_system(init_health_bar)
        //                     .with_system(setup_storage_display)
        //                     .with_system(setup_blueprint_display),
        //             )
        //             .add_system_set(
        //                 SystemSet::on_update(AppState::InGame)
        //                     .with_system(update_storage_display)
        //                     .with_system(update_blueprint_display)
        //                     .with_system(button_system)
        //                     .with_system(button_timer_system)
        //                     .with_system(recipe_hover_system),
        //             )
        //             .add_system_set(
        //                 SystemSet::on_update(AppState::InGame)
        //                     .with_system(update_health_display)
        //                     .with_system(animate),
        //             )
        //             .add_system_set(
        //                 SystemSet::on_update(AppState::InGame)
        //                     .with_system(set_recipe_global_transform)
        //                     .with_system(set_recipe_hovered_global_transform)
        //                     .with_system(set_storage_global_transform)
        //                     .with_system(set_blueprint_global_transform)
        //                     .after("camera"),
        //             );
    }
}
//
// #[derive(Component)]
// pub struct HealthBarDisplay;
//
// #[derive(Component)]
// pub struct HealthText;
//
// #[derive(Component)]
// pub struct HealthBarDisplayComponent {
//     pub cur_percent: f32,
//     displayed_percent: f32,
// }
//
// impl HealthBarDisplayComponent {
//     pub fn animate(&mut self, mut transform: Mut<Transform>) {
//         if self.displayed_percent != self.cur_percent {
//             self.displayed_percent = self.cur_percent.max(self.displayed_percent - 0.9);
//         }
//         let new_x = 350.0 * self.displayed_percent / 100.0;
//         transform.scale = Vec3::new(new_x, 15.0, 0.0).into();
//     }
// }
//
// pub fn spawn_health_bar(commands: &mut Commands) {
//     commands
//         .spawn_bundle(SpriteBundle {
//             transform: Transform {
//                 translation: Vec3::new(0.0, -50.0, 2.0),
//                 scale: Vec3::new(350.0, 15.0, 0.0).into(),
//                 ..Default::default()
//             },
//             sprite: Sprite {
//                 color: Color::rgb_u8(184, 248, 174),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .insert(HealthBarDisplayComponent {
//             cur_percent: 1.0,
//             displayed_percent: 1.0,
//         });
// }
//
// #[derive(Bundle)]
// pub struct HealthTextBundle {
//     #[bundle]
//     text: TextBundle,
// }
//
// impl HealthTextBundle {
//     pub fn new(asset_server: &Res<AssetServer>) -> Self {
//         HealthTextBundle {
//             text: TextBundle {
//                 text: Text {
//                     sections: vec![
//                         TextSection {
//                             value: "Health: ".to_string(),
//                             style: TextStyle {
//                                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                                 font_size: 40.0,
//                                 color: Color::rgb(0.5, 0.5, 1.0),
//                             },
//                         },
//                         TextSection {
//                             value: "".to_string(),
//                             style: TextStyle {
//                                 font: asset_server.load("fonts/FiraMono-Medium.ttf"),
//                                 font_size: 40.0,
//                                 color: Color::rgb(1.0, 0.5, 0.5),
//                             },
//                         },
//                     ],
//                     ..Default::default()
//                 },
//                 style: Style {
//                     position_type: PositionType::Absolute,
//                     position: Rect {
//                         top: Val::Px(5.0),
//                         left: Val::Px(5.0),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         }
//     }
// }
//
// #[derive(Component)]
// pub struct StorageBox;
//
// #[derive(Component)]
// pub struct BlueprintBox;
//
// #[derive(Default)]
// pub struct StorageUIs {
//     pub entities: std::vec::Vec<Entity>,
// }
//
// #[derive(Default)]
// pub struct BlueprintUIs {
//     pub entities: std::vec::Vec<Entity>,
//     pub res: std::vec::Vec<Entity>,
// }
//
// #[derive(Component)]
// pub struct StorageUI {
//     pub child: Type,
// }
//
// #[derive(Component)]
// pub struct BlueprintUI {
//     pub child: Type,
// }
//
// #[derive(Component)]
// pub struct StorageShape;
//
// #[derive(Component)]
// pub struct BlueprintShape;
//
// #[derive(Component)]
// pub struct RecipeDisplayButton {
//     pub clicked: bool,
//     pub timer: Timer,
// }
//
// #[derive(Default)]
// pub struct RecipeTableUi(pub Option<Entity>);
//
// #[derive(Component)]
// pub struct RecipeBoxUi {
//     pub entity: Option<Entity>,
//     pub id: usize,
// }
//
// #[derive(Component)]
// pub struct RecipeShape;
//
// #[derive(Component)]
// pub struct RecipeBox {
//     pub clicked: bool,
// }
//
// #[derive(Component)]
// struct RecipeHoveredBox(Vec3);
//
// #[derive(Default)]
// struct RecipeHoveredBoxUI(Option<Entity>);
//
// fn init_box(extents: Vec2, pos: Vec2) -> ShapeBundle {
//     let shape = shapes::Rectangle {
//         extents,
//         origin: RectangleOrigin::Center,
//     };
//     GeometryBuilder::build_as(
//         &shape,
//         DrawMode::Outlined {
//             fill_mode: FillMode::color(Color::rgba(0.9, 0.9, 0.9, 0.7)),
//             outline_mode: StrokeMode::new(Color::rgba(0.7, 0.7, 0.7, 0.7), 5.0),
//         },
//         Transform {
//             translation: Vec3::new(pos.x, pos.y, 1.0),
//             ..Default::default()
//         },
//     )
// }
//
// pub fn init_health_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn_bundle(NodeBundle {
//             style: Style {
//                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                 position_type: PositionType::Absolute,
//                 justify_content: JustifyContent::FlexStart,
//                 align_items: AlignItems::FlexEnd,
//                 ..Default::default()
//             },
//             color: Color::NONE.into(),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             parent
//                 .spawn_bundle(NodeBundle {
//                     style: Style {
//                         size: Size::new(Val::Px(100.0), Val::Px(10.0)),
//                         padding: Rect::all(Val::Px(2.0)),
//                         justify_content: JustifyContent::FlexStart,
//                         align_items: AlignItems::FlexEnd,
//                         ..Default::default()
//                     },
//                     color: Color::rgb(0.3, 0.3, 0.3).into(),
//                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//                     parent
//                         .spawn_bundle(NodeBundle {
//                             style: Style {
//                                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                                 ..Default::default()
//                             },
//                             color: Color::rgb_u8(184, 248, 174).into(),
//                             ..Default::default()
//                         })
//                         .insert(HealthBarDisplay);
//                 });
//         });
//
//     spawn_health_bar(&mut commands);
//     commands
//         .spawn_bundle(HealthTextBundle::new(&asset_server))
//         .insert(HealthText);
// }
//
// fn update_health_display(
//     app_state: Res<State<AppState>>,
//     mut query: Query<&mut Text, With<HealthText>>,
//     player_health: Query<&Health, With<Player>>,
//     mut health_bar: Query<&mut Style, With<HealthBarDisplay>>,
//     mut health_bar_component: Query<&mut HealthBarDisplayComponent>,
// ) {
//     if *app_state.current() == AppState::EndGame {
//         return;
//     }
//     let mut text = query.single_mut();
//     let health = player_health.single();
//     text.sections[1].value = format!("{}", health.hp);
//
//     let mut health_bar = health_bar.single_mut();
//     health_bar.size = Size::new(Val::Percent(health.hp as f32), Val::Percent(80.0));
//
//     let mut health_bar = health_bar_component.single_mut();
//     health_bar.cur_percent = health.hp as f32;
// }
//
// fn animate(mut health_bar_component: Query<(&mut Transform, &mut HealthBarDisplayComponent)>) {
//     let (transform, mut health_bar) = health_bar_component.single_mut();
//     health_bar.animate(transform);
// }
//
// pub fn init_ui(mut commands: Commands, asset_server: ResMut<AssetServer>) {
//     let mut entities = vec![];
//     for i in 1..10 {
//         let id = if i >= OUTLINES.len() { 0 } else { i };
//         let e = commands
//             .spawn_bundle(OUTLINES[id](Usage::Storage))
//             .insert(RecipeShape {})
//             .id();
//         entities.push(e);
//     }
//     commands
//         .spawn_bundle(NodeBundle {
//             style: Style {
//                 size: Size::new(Val::Px(200.0), Val::Px(200.0)),
//                 position_type: PositionType::Absolute,
//                 flex_direction: FlexDirection::Column,
//                 justify_content: JustifyContent::FlexStart,
//                 align_items: AlignItems::Center,
//                 position: Rect {
//                     left: Val::Percent(20.0),
//                     bottom: Val::Percent(3.0),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//             color: Color::rgba(0.6, 0.6, 0.6, 0.0).into(),
//             ..Default::default()
//         })
//         .with_children(|parent| {
//             // button with text
//             parent
//                 .spawn_bundle(NodeBundle {
//                     style: Style {
//                         size: Size::new(Val::Px(100.0), Val::Px(50.0)),
//                         // center button
//                         margin: Rect::all(Val::Auto),
//                         // horizontally center child text
//                         justify_content: JustifyContent::Center,
//                         // vertically center child text
//                         align_items: AlignItems::Center,
//                         position_type: Absolute,
//                         border: Rect::all(Val::Px(5.0)),
//                         ..Default::default()
//                     },
//                     color: Color::hsla(50.0, 1.0, 0.5, 1.0).into(),
//                     ..Default::default()
//                 })
//                 .with_children(|parent| {
//                     parent
//                         .spawn_bundle(ButtonBundle {
//                             style: Style {
//                                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                                 justify_content: JustifyContent::Center,
//                                 align_items: AlignItems::Center,
//                                 ..Default::default()
//                             },
//                             color: Color::hsla(50.0, 1.0, 0.6, 1.0).into(),
//                             ..Default::default()
//                         })
//                         .insert(RecipeDisplayButton {
//                             clicked: false,
//                             timer: Timer::from_seconds(0.3, false),
//                         })
//                         .with_children(|parent| {
//                             parent.spawn_bundle(TextBundle {
//                                 text: Text::with_section(
//                                     "Recipes",
//                                     TextStyle {
//                                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                                         font_size: 20.0,
//                                         color: Color::rgb(0.2, 0.2, 0.2),
//                                     },
//                                     Default::default(),
//                                 ),
//                                 ..Default::default()
//                             });
//                         });
//                 });
//
//             // recipe book
//             parent
//                 .spawn_bundle(NodeBundle {
//                     style: Style {
//                         size: Size::new(Val::Px(200.0), Val::Px(200.0)),
//                         position_type: PositionType::Absolute,
//                         flex_direction: FlexDirection::ColumnReverse,
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         position: Rect {
//                             bottom: Val::Percent(-120.0),
//                             ..Default::default()
//                         },
//                         ..Default::default()
//                     },
//                     color: Color::rgba(0.6, 0.6, 0.6, 0.0).into(),
//                     ..Default::default()
//                 })
//                 .insert(RecipeBox { clicked: false })
//                 .with_children(|parent| {
//                     let num_rows = 3;
//                     let num_cols = 3;
//                     for r in 0..num_rows {
//                         parent
//                             .spawn_bundle(NodeBundle {
//                                 style: Style {
//                                     size: Size::new(
//                                         Val::Percent(100.0),
//                                         Val::Percent(100.0 / num_rows as f32),
//                                     ),
//                                     // margin: Rect::all(Val::Percent(5.0)),
//                                     border: Rect::all(Val::Px(5.0)),
//                                     flex_direction: FlexDirection::Row,
//                                     justify_content: JustifyContent::Center,
//                                     align_items: AlignItems::Center,
//                                     ..Default::default()
//                                 },
//                                 color: Color::NONE.into(),
//                                 ..Default::default()
//                             })
//                             .with_children(|parent| {
//                                 for c in 0..num_cols {
//                                     let id = r * num_cols + c + 1;
//                                     parent
//                                         .spawn_bundle(ButtonBundle {
//                                             style: Style {
//                                                 size: Size::new(
//                                                     Val::Percent(100.0 / num_cols as f32),
//                                                     Val::Percent(100.0),
//                                                 ),
//                                                 margin: Rect::all(Val::Px(5.0)),
//                                                 border: Rect::all(Val::Px(5.0)),
//                                                 ..Default::default()
//                                             },
//                                             color: Color::rgba(0.6, 0.6, 0.6, 0.0).into(),
//                                             ..Default::default()
//                                         })
//                                         .insert(RecipeBoxUi {
//                                             entity: Some(entities[id - 1]),
//                                             id,
//                                         });
//                                 }
//                             });
//                     }
//                 });
//         });
// }
//
// pub fn button_timer_system(time: Res<Time>, mut q: Query<&mut RecipeDisplayButton>) {
//     for mut button in q.iter_mut() {
//         button.timer.tick(time.delta());
//     }
// }
//
// pub fn button_system(
//     mut commands: Commands,
//     mut interaction_query: Query<
//         (&Interaction, &mut UiColor, &mut RecipeDisplayButton),
//         Changed<Interaction>,
//     >,
//     mut recipe_box_query: Query<(Entity, &mut Style), With<RecipeBox>>,
// ) {
//     for (interaction, mut color, mut button) in interaction_query.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 if button.timer.finished() {
//                     let c = color.0.as_hlsa_f32();
//                     color.0 = match button.clicked {
//                         true => Color::hsl(c[0], c[1], 0.6).into(),
//                         false => Color::hsl(c[0], c[1], 0.45).into(),
//                     };
//                     let (entity, style) = recipe_box_query.single_mut();
//                     let mut style_end = style.clone();
//                     style_end.position.bottom = match button.clicked {
//                         true => Val::Percent(-120.0),
//                         false => Val::Percent(20.0),
//                     };
//                     commands.spawn().insert(Animation {
//                         entity,
//                         start: style.clone(),
//                         end: style_end,
//                         timer: Timer::from_seconds(0.2, false),
//                     });
//                     button.clicked = !button.clicked;
//                     button.timer.reset();
//                 }
//             }
//             Interaction::Hovered => {
//                 if !button.clicked {
//                     let c = color.0.as_hlsa_f32();
//                     color.0 = Color::hsl(c[0], c[1], 0.7).into();
//                 }
//             }
//             Interaction::None => {
//                 if !button.clicked {
//                     let c = color.0.as_hlsa_f32();
//                     color.0 = Color::hsl(c[0], c[1], 0.6).into();
//                 }
//             }
//         }
//     }
// }
//
// pub fn rounded_rect(pos: Vec2) -> ShapeBundle {
//     // let shape = shapes::SvgPathShape {
//     //     svg_path_string: "M -112.5 -60 H 112.5 A 22.5 22.5 90 0 1 135 -37.5 V 12.5 A 22.5 22.5 90 0 1 112.5 35 H -112.5 A 22.5 22.5 90 0 1 -135 12.5 V -37.5 A 22.5 22.5 90 0 1 -112.5 -60".into(),
//     //     svg_doc_size_in_px: Vec2::new(0., 0.)
//     // };
//     let shape = shapes::Rectangle {
//         extents: Vec2::new(200.0, 80.0),
//         origin: RectangleOrigin::Center,
//     };
//     GeometryBuilder::build_as(
//         &shape,
//         DrawMode::Fill(FillMode::color(Color::rgba(0.1, 0.1, 0.1, 0.7))),
//         Transform {
//             translation: Vec3::new(pos.x, pos.y, 12.0),
//             ..Default::default()
//         },
//     )
// }
//
// fn recipe_hover_system(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     table_inverse: Res<TableInverse>,
//     button_query: Query<&RecipeDisplayButton>,
//     mut interaction_query: Query<
//         (&Interaction, &mut RecipeBoxUi, &GlobalTransform),
//         Changed<Interaction>,
//     >,
//     hovered_query: Query<Entity, With<RecipeHoveredBox>>,
// ) {
//     let button_query = button_query.single();
//     if button_query.timer.finished() {
//         for (interaction, ui, global_pos) in interaction_query.iter_mut() {
//             match *interaction {
//                 Interaction::Hovered => {
//                     let header_style = TextStyle {
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         font_size: 20.0,
//                         color: Color::WHITE,
//                     };
//                     let text_style = TextStyle {
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         font_size: 15.0,
//                         color: Color::WHITE,
//                     };
//                     let text_alignment = TextAlignment {
//                         vertical: VerticalAlign::Center,
//                         horizontal: HorizontalAlign::Center,
//                     };
//                     let _e = commands
//                         .spawn_bundle(rounded_rect(Vec2::new(-10000.0, -10000.0)))
//                         .insert(RecipeHoveredBox(Vec3::new(
//                             global_pos.translation.x,
//                             global_pos.translation.y + 80.0,
//                             12.0,
//                         )))
//                         .with_children(|parent| {
//                             parent.spawn_bundle(Text2dBundle {
//                                 text: Text::with_section(
//                                     "Ingredient",
//                                     header_style,
//                                     text_alignment,
//                                 ),
//                                 transform: Transform::from_translation(Vec3::new(0.0, 25.0, 13.0)),
//                                 ..Default::default()
//                             });
//                             let y = -5.0;
//                             if let Some(v) = table_inverse.0.get(&ui.id) {
//                                 let extents = Vec2::new(20.0, 20.0);
//                                 let interval = 20.0;
//                                 let mut cur_x =
//                                     (v.len() - 1) as f32 * (-interval - extents.x) / 2.0;
//                                 for (id, num) in v.iter() {
//                                     parent
//                                         .spawn_bundle((
//                                             Transform::from_translation(Vec3::new(cur_x, y, 13.0)),
//                                             GlobalTransform::default(),
//                                         ))
//                                         .with_children(|parent| {
//                                             parent
//                                                 .spawn_bundle(OUTLINES[*id as usize](Usage::Storage));
//                                         });
//                                     cur_x += extents.x;
//                                     parent.spawn_bundle(Text2dBundle {
//                                         text: Text::with_section(
//                                             format!("??{}", num.to_string()),
//                                             text_style.clone(),
//                                             text_alignment,
//                                         ),
//                                         transform: Transform::from_translation(Vec3::new(
//                                             cur_x, y, 13.0,
//                                         )),
//                                         ..Default::default()
//                                     });
//                                     cur_x += interval;
//                                 }
//                             } else {
//                                 parent.spawn_bundle(Text2dBundle {
//                                     text: Text::with_section("None", text_style, text_alignment),
//                                     transform: Transform::from_translation(Vec3::new(0.0, y, 13.0)),
//                                     ..Default::default()
//                                 });
//                             }
//                         })
//                         .id();
//                 }
//                 Interaction::None => {
//                     if !hovered_query.is_empty() {
//                         for e in hovered_query.iter() {
//                             commands.entity(e).despawn_recursive();
//                         }
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
// }
//
// fn set_recipe_hovered_global_transform(
//     wnds: Res<Windows>,
//     mut q: QuerySet<(
//         QueryState<&Transform, With<MainCamera>>,
//         QueryState<(&mut Transform, &RecipeHoveredBox)>,
//     )>,
// ) {
//     let wnd = wnds.get_primary().unwrap();
//     let camera_pos = q.q0().single();
//     let mut x = camera_pos.translation.x;
//     let mut y = camera_pos.translation.y;
//
//     let mut q1 = q.q1();
//     for (mut pos, hovered_box_pos) in q1.iter_mut() {
//         let offset_x = wnd.width() as f32 / 2.0 - hovered_box_pos.0.x;
//         let offset_y = wnd.height() as f32 / 2.0 - hovered_box_pos.0.y;
//         x -= offset_x;
//         y -= offset_y;
//
//         pos.translation.x = x;
//         pos.translation.y = y;
//     }
// }
//
// pub fn set_recipe_global_transform(
//     wnds: Res<Windows>,
//     mut q: QuerySet<(
//         QueryState<&Transform, With<MainCamera>>,
//         QueryState<(&Transform, &GlobalTransform, &RecipeBoxUi)>,
//         QueryState<&mut Transform, With<RecipeShape>>,
//     )>,
// ) {
//     let wnd = wnds.get_primary().unwrap();
//     let camera_pos = q.q0().single();
//     let x = camera_pos.translation.x;
//     let y = camera_pos.translation.y;
//
//     let q1 = q.q1();
//     let mut v = vec![];
//     for (_pos, global_pos, ui) in q1.iter() {
//         v.push((
//             x - (wnd.width() as f32 / 2.0 - global_pos.translation.x),
//             y - (wnd.height() as f32 / 2.0 - global_pos.translation.y),
//             ui.entity.unwrap(),
//         ));
//     }
//     let mut q2 = q.q2();
//     for (x, y, e) in v.into_iter() {
//         let mut pos = q2.get_mut(e).unwrap();
//         pos.translation.x = x;
//         pos.translation.y = y;
//     }
// }
//
// fn setup_storage_display(mut commands: Commands, mut storage_ui: ResMut<StorageUIs>) {
//     let storage_ui = storage_ui.as_mut();
//     commands
//         .spawn_bundle((
//             Transform {
//                 translation: Vec3::new(0.0, 0.0, 3.0),
//                 ..Default::default()
//             },
//             GlobalTransform::default(),
//         ))
//         .insert(StorageBox)
//         .with_children(|parent| {
//             let extents = Vec2::new(40.0, 40.0);
//             let interval = 20.0;
//             let mut cur_x = (STORAGE_SIZE - 1) as f32 * (-interval - extents.x) / 2.0;
//             for _ in 0..STORAGE_SIZE {
//                 let e = parent
//                     .spawn_bundle(init_box(extents, Vec2::new(cur_x, 0.0)))
//                     .insert(StorageUI { child: Type::Empty })
//                     .with_children(|_| {})
//                     .id();
//                 storage_ui.entities.push(e);
//                 cur_x += extents.x + interval;
//             }
//         });
// }
//
// fn update_storage_display(
//     mut commands: Commands,
//     storage_uis: Res<StorageUIs>,
//     storage_in_hand: ResMut<StorageInHand>,
//     storage: Query<&Storage>,
//     mut q: Query<(&mut Children, &mut StorageUI)>,
//     mut transform_query: Query<&mut Transform, With<StorageShape>>,
// ) {
//     let storage = storage.single();
//     if storage_in_hand.cur != storage_in_hand.prev {
//         if let Some(i) = storage_in_hand.prev {
//             let (children, _) = q.get_mut(storage_uis.entities[i]).unwrap();
//             for child in children.iter() {
//                 if let Ok(mut transform) = transform_query.get_mut(*child) {
//                     transform.rotation = Quat::from_rotation_y(0.0);
//                 }
//             }
//         }
//     }
//     if let Some(i) = storage_in_hand.cur {
//         let (children, _) = q.get_mut(storage_uis.entities[i]).unwrap();
//         for child in children.iter() {
//             if let Ok(mut transform) = transform_query.get_mut(*child) {
//                 transform.rotation = transform.rotation.mul_quat(Quat::from_rotation_y(0.03));
//             }
//         }
//     }
//     for (i, &id) in storage.items.iter().enumerate() {
//         let parent = storage_uis.entities[i];
//         let (_, mut storage_ui): (_, Mut<StorageUI>) = q.get_mut(parent).unwrap();
//         if id != storage_ui.child {
//             commands.entity(parent).despawn_descendants();
//             if id != Type::Empty {
//                 let f = OUTLINES[(id) as usize];
//                 let child = commands
//                     .spawn_bundle(f(Usage::Storage))
//                     .insert(StorageShape {})
//                     .id();
//                 commands.entity(parent).push_children(&[child]);
//             }
//             storage_ui.child = id;
//         }
//     }
// }
//
// fn set_storage_global_transform(
//     wnds: Res<Windows>,
//     mut q: QuerySet<(
//         QueryState<&Transform, With<MainCamera>>,
//         QueryState<&mut Transform, With<StorageBox>>,
//     )>,
// ) {
//     let wnd = wnds.get_primary().unwrap();
//     let camera_pos = q.q0().single();
//     let x = camera_pos.translation.x;
//     let y = camera_pos.translation.y - wnd.height() as f32 / 2.3;
//
//     let mut q1 = q.q1();
//     let mut storage_pos = q1.single_mut();
//     storage_pos.translation.x = x;
//     storage_pos.translation.y = y;
// }
//
// fn setup_blueprint_display(mut commands: Commands, mut blueprint_uis: ResMut<BlueprintUIs>) {
//     let blueprint_uis = blueprint_uis.as_mut();
//     commands
//         .spawn_bundle((
//             Transform {
//                 translation: Vec3::new(0.0, 0.0, 3.0),
//                 ..Default::default()
//             },
//             GlobalTransform::default(),
//         ))
//         .insert(BlueprintBox)
//         .with_children(|parent| {
//             let extents = Vec2::new(40.0, 40.0);
//             let interval = 0.0;
//             let mut cur_x = (BLUEPRINT_SIZE - 1) as f32 * (-interval - extents.x) / 2.0;
//             for _ in 0..BLUEPRINT_SIZE {
//                 let e = parent
//                     .spawn_bundle(init_box(extents, Vec2::new(cur_x, 0.0)))
//                     .insert(BlueprintUI { child: Type::Empty })
//                     .with_children(|_| {})
//                     .id();
//                 blueprint_uis.entities.push(e);
//                 cur_x += extents.x + interval;
//             }
//             cur_x += 10.0;
//             let extents = Vec2::new(40.0, 40.0);
//             let e = parent
//                 .spawn_bundle(init_box(extents, Vec2::new(cur_x, 0.0)))
//                 .insert(BlueprintUI { child: Type::Empty })
//                 .with_children(|_| {})
//                 .id();
//             blueprint_uis.res.push(e);
//         });
// }
//
// fn update_blueprint_box(
//     commands: &mut Commands,
//     parent: Entity,
//     parent_ui: &mut BlueprintUI,
//     id: Type,
// ) {
//     if id != parent_ui.child {
//         if id == Type::Empty {
//             commands.entity(parent).despawn_descendants();
//         } else {
//             let f = OUTLINES[id as usize];
//             let child = commands
//                 .spawn_bundle(f(Usage::Storage))
//                 .insert(BlueprintShape {})
//                 .id();
//             commands.entity(parent).push_children(&[child]);
//         }
//         parent_ui.child = id;
//     }
// }
//
// fn update_blueprint_display(
//     mut commands: Commands,
//     table: Res<Table>,
//     blueprint_uis: Res<BlueprintUIs>,
//     blueprint: Query<&Blueprint>,
//     mut q: Query<(&mut Children, &mut BlueprintUI)>,
//     // mut transform_query: Query<&mut Transform, With<BlueprintShape>>
// ) {
//     let bp = blueprint.single();
//     for (i, &id) in bp.items.iter().enumerate() {
//         let parent = blueprint_uis.entities[i];
//         let (_, mut blueprint_ui): (_, Mut<BlueprintUI>) = q.get_mut(parent).unwrap();
//         update_blueprint_box(&mut commands, parent, blueprint_ui.as_mut(), id);
//     }
//     let parent = blueprint_uis.res[0];
//     let (_, mut blueprint_ui): (_, Mut<BlueprintUI>) = q.get_mut(parent).unwrap();
//     let k: std::vec::Vec<(Type, usize)> = bp.clone().into();
//     match table.0.get(&k) {
//         Some(&id) => {
//             update_blueprint_box(&mut commands, parent, blueprint_ui.as_mut(), id);
//             blueprint_ui.child = id;
//         }
//         None => {
//             commands.entity(parent).despawn_descendants();
//             blueprint_ui.child = Type::Empty;
//         }
//     }
// }
//
// fn set_blueprint_global_transform(
//     wnds: Res<Windows>,
//     mut q: QuerySet<(
//         QueryState<&Transform, With<MainCamera>>,
//         QueryState<&mut Transform, With<BlueprintBox>>,
//     )>,
// ) {
//     let wnd = wnds.get_primary().unwrap();
//     let camera_pos = q.q0().single();
//     let x = camera_pos.translation.x - wnd.width() as f32 / 2.5;
//     let y = camera_pos.translation.y - wnd.height() as f32 / 2.3;
//
//     let mut q1 = q.q1();
//     let mut blueprint_pos = q1.single_mut();
//     blueprint_pos.translation.x = x;
//     blueprint_pos.translation.y = y;
// }
