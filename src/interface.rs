use crate::{mana::Mana, player::Player};
use bevy::prelude::*;

struct PlayerManaText;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn.system())
            .add_system(update_mana_text_value.system());
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/pinch.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Mana: ", text_style.clone(), Default::default()),
            ..Default::default()
        })
        .insert(PlayerManaText);
}

fn update_mana_text_value(
    mut player_mana_text_query: Query<&mut Text, With<PlayerManaText>>,
    player_mana_query: Query<&Mana, With<Player>>,
) {
    let mut player_mana_text = player_mana_text_query.single_mut().unwrap();
    let player_mana = player_mana_query.single().unwrap();

    player_mana_text.sections[0].value = format!("Mana: {} / {}", player_mana.points, player_mana.max_points);
}
