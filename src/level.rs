use crate::{
    creature::{Creature, CreatureBundle},
    enemy::Enemy,
    player::Player,
    sprite::Sprite,
    AppState,
};
use bevy::prelude::*;

const PLAYER_TRANSLATION: (f32, f32, f32) = (-80.0, 0.0, 0.0);
const GOBLIN_TRANSLATIONS: [(f32, f32, f32); 2] = [(80.0, 30.0, 0.0), (80.0, -30.0, 0.0)];

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_system))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_system));
    }
}

fn spawn_system(mut commands: Commands, texture_atlases: Res<Assets<TextureAtlas>>) {
    commands
        .spawn_bundle(CreatureBundle::new(160, 100))
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.get_handle(Sprite::SHEET_PATH),
            sprite: TextureAtlasSprite::new(Sprite::Player.index()),
            transform: Transform::from_translation(Vec3::from(PLAYER_TRANSLATION)),
            ..Default::default()
        });

    for goblin_translation in GOBLIN_TRANSLATIONS {
        commands
            .spawn_bundle(CreatureBundle::new(80, 40))
            .insert(Enemy)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlases.get_handle(Sprite::SHEET_PATH),
                sprite: TextureAtlasSprite::new(Sprite::Goblin.index()),
                transform: Transform::from_translation(Vec3::from(goblin_translation)),
                ..Default::default()
            });
    }
}

fn despawn_system(mut commands: Commands, query: Query<Entity, With<Creature>>) {
    for creature_entity in query.iter() {
        commands.entity(creature_entity).despawn();
    }
}
