mod animation;
mod bundle;
mod camera;
mod component;
mod end_game;
mod in_game;
mod magic;
mod particle;
mod shape_mod;
mod synthesis;
mod ui;

use bundle::*;
use camera::*;
use end_game::*;
use in_game::*;
use particle::*;
use shape_mod::*;

use crate::animation::AnimationPlugin;
use bevy::asset::{HandleId, LoadState};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::FilterMode;
use bevy::utils::HashMap;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

// size(pixels) = RAPIER_SCALE * size(meters)
const RAPIER_TO_BEVY: f32 = 10.0;
const RAPIER_TO_LYON: f32 = 10.0;

const BOUNDARY_HORIZONTAL: f32 = 192.0;
const BOUNDARY_VERTICAL: f32 = 108.0;
const OFFSET_HORIZONTAL: f32 = 50.0;
const OFFSET_VERTICAL: f32 = 50.0;

fn main() {
    let mut app = App::new();
    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }
    app.add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(CameraPlugin)
        .add_plugin(ParticlePlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(InGamePlugin)
        .add_plugin(EndGamePlugin)
        .add_state(AppState::Setup)
        .init_resource::<Msaa>()
        .init_resource::<SpriteHandles>()
        .init_resource::<SpriteAtlasHandle>()
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_exit(AppState::Setup).with_system(setup_game))
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Setup,
    InGame,
    EndGame,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Default)]
pub struct SpriteAtlasHandle {
    pub handle: Handle<TextureAtlas>,
    map: HashMap<HandleId, usize>,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("sprites").unwrap();
}

fn check_textures(
    mut app_state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    mut sprite_atlas_handle: ResMut<SpriteAtlasHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for handle in &sprite_handles.handles {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let texture_atlas_texture = texture_atlas.texture.clone();
        let image = textures.get_mut(texture_atlas_texture.clone()).unwrap();
        image.sampler_descriptor.min_filter = FilterMode::Linear;
        image.sampler_descriptor.mag_filter = FilterMode::Linear;

        for handle in &sprite_handles.handles {
            let handle_id = handle.id;
            let idx = texture_atlas
                .get_texture_index(&handle.clone_weak().typed::<Image>())
                .unwrap();
            sprite_atlas_handle.map.insert(handle_id, idx);
        }
        let atlas_handle = texture_atlases.add(texture_atlas);
        sprite_atlas_handle.handle = atlas_handle;

        // commands.spawn_bundle(SpriteBundle {
        //     texture: texture_atlas_texture.clone(),
        //     transform: Transform::from_xyz(0.0, 0.0, -1.0),
        //     ..Default::default()
        // });

        let _ = app_state.set(AppState::InGame).unwrap();
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut config: ResMut<RapierConfiguration>,
) {
    config.gravity = Vec2::new(0.0, 0.0).into();
    config.scale = RAPIER_TO_BEVY;
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        texture: asset_server.load("bg.png"),
        ..Default::default()
    });
    spawn_boundary(&mut commands);

    commands.insert_resource(EntityInRange {
        cur: None,
        prev: None,
    });
    commands.insert_resource(EntityInHand { entity: None });
    commands.insert_resource(SpawnTimer(Timer::from_seconds(1.0, true)))
}

fn spawn_boundary(commands: &mut Commands) {
    let half_m = BOUNDARY_VERTICAL / 2.0 + OFFSET_VERTICAL;
    let half_n = BOUNDARY_HORIZONTAL / 2.0 + OFFSET_HORIZONTAL;
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(half_n, OFFSET_VERTICAL),
        Vec2::new(0.0, half_m),
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(half_n, OFFSET_VERTICAL),
        Vec2::new(0.0, -half_m),
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(OFFSET_HORIZONTAL, half_m),
        Vec2::new(half_n, 0.0),
    ));
    commands.spawn_bundle(StaticBundle::new_rect(
        Vec2::new(OFFSET_HORIZONTAL, half_m),
        Vec2::new(-half_n, 0.0),
    ));
}
