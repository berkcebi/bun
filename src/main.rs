mod ability;
mod creature;
mod critical;
mod effect;
mod enemy;
mod health;
mod interface;
mod level;
mod mana;
mod player;
mod position;
mod sprite;
mod target;
mod zone;

use ability::AbilityPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use effect::EffectPlugin;
use interface::InterfacePlugins;
use level::LevelPlugin;
use mana::ManaPlugin;
use player::PlayerPlugin;
use position::PositionPlugin;
use sprite::Sprite;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const CAMERA_SCALE: f32 = 1.0 / 2.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Bun".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            // Turn vsync on to limit frame rate and reduce power consumption while debugging.
            vsync: true,
            ..Default::default()
        })
        .add_state(AppState::Game)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(InterfacePlugins)
        .add_plugin(AbilityPlugin)
        .add_plugin(EffectPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(ManaPlugin)
        .add_plugin(PositionPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_sheet_image_handle = asset_server.load(Sprite::SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        sprite_sheet_image_handle,
        Vec2::splat(Sprite::SIZE),
        Sprite::SHEET_COLUMNS,
        Sprite::SHEET_ROWS,
    );
    // Use path as handle identifier.
    let _ = texture_atlases.set(Sprite::SHEET_PATH, texture_atlas);

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = CAMERA_SCALE;
    commands.spawn_bundle(camera_bundle);
}
