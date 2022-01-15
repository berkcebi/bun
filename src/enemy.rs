use crate::{creature::CreatureBundle, sprite::Sprite};
use bevy::prelude::*;

const GOBLIN_TRANSLATIONS: [(f32, f32, f32); 2] = [(80.0, 30.0, 0.0), (80.0, -30.0, 0.0)];

pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn.system());
    }
}

fn spawn(mut commands: Commands, texture_atlases: Res<Assets<TextureAtlas>>) {
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
