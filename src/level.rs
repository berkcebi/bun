use crate::{
    creature::{Creature, CreatureBundle},
    enemy::Enemy,
    health::Health,
    player::Player,
    sprite::Sprite,
    zone::Zone,
    AppState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_TRANSLATION: (f32, f32, f32) = (-80.0, 0.0, 0.0);
const GOBLIN_TRANSLATIONS: [(f32, f32, f32); 2] = [(80.0, 30.0, 0.0), (80.0, -30.0, 0.0)];
const ZONE_COLUMNS: usize = 22;
const ZONE_ROWS: usize = 16;

/// Resource to keep track of the level's result. Set to `LevelResult::None` while the level is in progress.
pub enum LevelResult {
    None,
    Won,
    Lost,
}

#[derive(Component)]
struct Tile;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelResult::None)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_system))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(end_system))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_system));
    }
}

fn spawn_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut level_result: ResMut<LevelResult>,
) {
    *level_result = LevelResult::None;

    let zone = Zone::new(ZONE_COLUMNS, ZONE_ROWS);
    for (x, row_tiles) in zone.tiles.iter().enumerate() {
        for (y, tile) in row_tiles.iter().enumerate() {
            if let Some(tile) = tile {
                let tile_position = zone.tile_position(x, y);
                let entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlases.get_handle(Sprite::SHEET_PATH),
                        sprite: TextureAtlasSprite::new(tile.sprite.index()),
                        transform: Transform::from_translation(tile_position.extend(0.0)),
                        ..Default::default()
                    })
                    .insert(Tile)
                    .id();

                if tile.is_obstructed {
                    let cuboid_half_extent = crate::zone::Tile::SIZE / 2.0;
                    commands.entity(entity).insert_bundle(ColliderBundle {
                        shape: ColliderShape::cuboid(cuboid_half_extent, cuboid_half_extent).into(),
                        position: (tile_position, 0.0).into(),
                        ..Default::default()
                    });
                }
            }
        }
    }

    commands
        .spawn_bundle(CreatureBundle::new(160, 100))
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.get_handle(Sprite::SHEET_PATH),
            sprite: TextureAtlasSprite::new(Sprite::Player.index()),
            transform: Transform::from_translation(PLAYER_TRANSLATION.into()),
            ..Default::default()
        });

    for goblin_translation in GOBLIN_TRANSLATIONS {
        commands
            .spawn_bundle(CreatureBundle::new(80, 40))
            .insert(Enemy)
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlases.get_handle(Sprite::SHEET_PATH),
                sprite: TextureAtlasSprite::new(Sprite::Goblin.index()),
                transform: Transform::from_translation(goblin_translation.into()),
                ..Default::default()
            });
    }
}

fn end_system(
    mut app_state: ResMut<State<AppState>>,
    mut level_result: ResMut<LevelResult>,
    player_query: Query<&Health, With<Player>>,
    enemy_query: Query<&Health, With<Enemy>>,
) {
    let player_health = player_query.single();
    if player_health.points == 0 {
        *level_result = LevelResult::Lost;
        app_state.set(AppState::Menu).unwrap();
    } else if enemy_query
        .iter()
        .all(|enemy_health| enemy_health.points == 0)
    {
        *level_result = LevelResult::Won;
        app_state.set(AppState::Menu).unwrap();
    }
}

fn despawn_system(
    mut commands: Commands,
    tile_query: Query<Entity, With<Tile>>,
    creature_query: Query<Entity, With<Creature>>,
) {
    for entity in tile_query.iter().chain(creature_query.iter()) {
        commands.entity(entity).despawn();
    }
}
