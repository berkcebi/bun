use crate::{level::LevelResult, AppState};
use bevy::prelude::*;

const FONT_PATH: &str = "fonts/04b03.ttf";
const FONT_SIZE: f32 = 12.0;

#[derive(Component)]
struct Menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(spawn_system))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(handle_keyboard_input_system),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_system));
    }
}

fn spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_result: Res<LevelResult>,
) {
    let text_style = TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: FONT_SIZE,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    let text_value = match *level_result {
        LevelResult::None => "Press Return to start.",
        LevelResult::Won => "Not bad, press Return to restart.",
        LevelResult::Lost => "Wrecked! Press Return to restart.",
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(text_value.to_string(), text_style, text_alignment),
            ..Default::default()
        })
        .insert(Menu);
}

fn despawn_system(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        app_state.set(AppState::Game).unwrap();
    }
}
