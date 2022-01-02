mod ability;
mod creature;
mod critical;
mod effect;
mod health;
mod interface;
mod mana;
mod player;

use ability::AbilityPlugin;
use bevy::prelude::*;
use effect::EffectPlugin;
use interface::InterfacePlugin;
use mana::ManaPlugin;
use player::PlayerPlugin;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const CAMERA_SCALE: f32 = 1.0 / 2.0;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Bun".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AbilityPlugin)
        .add_plugin(EffectPlugin)
        .add_plugin(InterfacePlugin)
        .add_plugin(ManaPlugin)
        .add_plugin(PlayerPlugin)
        .add_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = CAMERA_SCALE;
    commands.spawn_bundle(camera_bundle);
}
