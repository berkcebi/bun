mod ability;
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
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
