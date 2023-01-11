use crate::{player::PlayerTargetChanged, AppState};
use bevy::prelude::*;

#[derive(Component)]
struct TargetIndicator;

pub struct TargetIndicatorPlugin;

impl Plugin for TargetIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_system))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_system));
    }
}

fn spawn_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_target_changed_event_reader: EventReader<PlayerTargetChanged>,
    query: Query<Entity, With<TargetIndicator>>,
) {
    let player_target_changed = match player_target_changed_event_reader.iter().last() {
        Some(result) => result,
        None => return,
    };

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    let target_entity = match player_target_changed.target_entity {
        Some(result) => result,
        None => return,
    };

    let entity = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlases.get_handle(crate::Sprite::SHEET_PATH),
                sprite: TextureAtlasSprite::new(crate::Sprite::TargetIndicator.index()),
                ..default()
            },
            TargetIndicator,
        ))
        .id();

    commands.entity(target_entity).add_child(entity);
}

fn despawn_system(mut commands: Commands, query: Query<Entity, With<TargetIndicator>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
