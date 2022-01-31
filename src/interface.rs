use crate::{
    ability::CastAbility,
    enemy::Enemy,
    health::Health,
    mana::Mana,
    player::{Player, PlayerTargetChanged},
    CAMERA_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use bevy::{ecs::component::Component, prelude::*};

const WIDTH: f32 = WINDOW_WIDTH * CAMERA_SCALE;
const HEIGHT: f32 = WINDOW_HEIGHT * CAMERA_SCALE;

const TRANSLATION_Z: f32 = 50.0;

const PLAYER_BAR_WIDTH: f32 = 96.0;
const PLAYER_CAST_BAR_WIDTH: f32 = 144.0;
const PLAYER_BAR_HEIGHT: f32 = 16.0;
const ENEMY_BAR_HEIGHT: f32 = 4.0;
const PLAYER_BAR_MARGIN: f32 = 16.0;
const ENEMY_BAR_MARGIN: f32 = 1.0;
const BAR_BACKGROUND_COLOR_ALPHA: f32 = 0.25;
const BAR_TEXT_FONT_PATH: &str = "fonts/04b03.ttf";
const BAR_TEXT_FONT_SIZE: f32 = 12.0;
const BAR_TEXT_VERTICAL_OFFSET: f32 = -0.5;

const HEALTH_BAR_COLOR: Color = Color::rgb(231.0 / 255.0, 39.0 / 255.0, 37.0 / 255.0);
const MANA_BAR_COLOR: Color = Color::rgb(43.0 / 255.0, 102.0 / 255.0, 201.0 / 255.0);
const CAST_BAR_COLOR: Color = Color::rgb(1.0, 240.0 / 255.0, 0.0);

trait Progressive {
    fn get_progress(&self) -> f32;
    fn get_progress_description(&self) -> String;
}

impl Progressive for Health {
    fn get_progress(&self) -> f32 {
        self.points as f32 / self.max_points as f32
    }

    fn get_progress_description(&self) -> String {
        format!("{}/{}", self.points, self.max_points)
    }
}

impl Progressive for Mana {
    fn get_progress(&self) -> f32 {
        self.points as f32 / self.max_points as f32
    }

    fn get_progress_description(&self) -> String {
        format!("{}/{}", self.points, self.max_points)
    }
}

impl Progressive for CastAbility {
    fn get_progress(&self) -> f32 {
        self.duration_timer.percent()
    }

    fn get_progress_description(&self) -> String {
        self.ability.name.to_string()
    }
}

trait Bar: Component {
    type Type: Component + Progressive;

    fn entity(&self) -> Entity;
}

impl Bar for HealthBar {
    type Type = Health;

    fn entity(&self) -> Entity {
        self.entity
    }
}

impl Bar for ManaBar {
    type Type = Mana;

    fn entity(&self) -> Entity {
        self.entity
    }
}

impl Bar for CastBar {
    type Type = CastAbility;

    fn entity(&self) -> Entity {
        self.entity
    }
}

#[derive(Component)]
struct HealthBar {
    entity: Entity,
}

#[derive(Component)]
struct ManaBar {
    entity: Entity,
}

#[derive(Component)]
struct CastBar {
    entity: Entity,
}

#[derive(Component)]
struct PlayerTargetIndicator;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_player_bars_system)
            .add_system(add_enemy_health_bars_system)
            .add_system(update_bar_text_system::<HealthBar>)
            .add_system(update_bar_indicator_system::<HealthBar>)
            .add_system(update_bar_text_system::<ManaBar>)
            .add_system(update_bar_indicator_system::<ManaBar>)
            .add_system(update_bar_text_system::<CastBar>)
            .add_system(update_bar_indicator_system::<CastBar>)
            .add_system(update_cast_bar_visibility_system)
            .add_system(handle_player_target_changed_system);
    }
}

fn add_player_bars_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Player>>,
) {
    let entity = match query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let bar_text_font = asset_server.load(BAR_TEXT_FONT_PATH);

    spawn_bar(
        HEALTH_BAR_COLOR,
        Vec3::new(
            WIDTH * -0.5 + PLAYER_BAR_WIDTH * 0.5 + PLAYER_BAR_MARGIN,
            HEIGHT * 0.5 - PLAYER_BAR_HEIGHT * 0.5 - PLAYER_BAR_MARGIN,
            TRANSLATION_Z,
        ),
        Vec2::new(PLAYER_BAR_WIDTH, PLAYER_BAR_HEIGHT),
        HealthBar { entity },
        &mut commands,
        Some(bar_text_font.clone()),
    );

    spawn_bar(
        MANA_BAR_COLOR,
        Vec3::new(
            WIDTH * -0.5 + PLAYER_BAR_WIDTH * 0.5 + PLAYER_BAR_MARGIN,
            HEIGHT * 0.5 - PLAYER_BAR_HEIGHT * 1.5 - PLAYER_BAR_MARGIN * 1.5,
            TRANSLATION_Z,
        ),
        Vec2::new(PLAYER_BAR_WIDTH, PLAYER_BAR_HEIGHT),
        ManaBar { entity },
        &mut commands,
        Some(bar_text_font.clone()),
    );

    spawn_bar(
        CAST_BAR_COLOR,
        Vec3::new(0.0, HEIGHT / -4.0, TRANSLATION_Z),
        Vec2::new(PLAYER_CAST_BAR_WIDTH, PLAYER_BAR_HEIGHT),
        CastBar { entity },
        &mut commands,
        Some(bar_text_font),
    );
}

fn add_enemy_health_bars_system(mut commands: Commands, query: Query<Entity, Added<Enemy>>) {
    for entity in query.iter() {
        let health_bar_entity = spawn_bar(
            HEALTH_BAR_COLOR,
            Vec3::new(
                0.0,
                (crate::Sprite::SIZE + ENEMY_BAR_HEIGHT) / 2.0 + ENEMY_BAR_MARGIN,
                TRANSLATION_Z,
            ),
            Vec2::new(crate::Sprite::SIZE, ENEMY_BAR_HEIGHT),
            HealthBar { entity },
            &mut commands,
            None,
        );

        commands.entity(entity).add_child(health_bar_entity);
    }
}

fn update_bar_text_system<T: Bar>(
    bar_query: Query<(&Children, &T)>,
    mut bar_children_text_query: Query<&mut Text>,
    progressive_query: Query<&T::Type>,
) {
    for (bar_children, bar) in bar_query.iter() {
        let progressive = match progressive_query.get(bar.entity()) {
            Ok(result) => result,
            Err(_) => continue,
        };

        for &bar_child in bar_children.iter() {
            let mut bar_text = match bar_children_text_query.get_mut(bar_child) {
                Ok(result) => result,
                Err(_) => continue,
            };

            bar_text.sections[0].value = progressive.get_progress_description();
        }
    }
}

fn update_bar_indicator_system<T: Bar>(
    bar_query: Query<(&Children, &Sprite, &T)>,
    mut bar_child_indicator_query: Query<(&mut Sprite, &mut Transform), Without<T>>,
    progressive_query: Query<&T::Type>,
) {
    for (bar_children, bar_sprite, bar) in bar_query.iter() {
        let progressive = match progressive_query.get(bar.entity()) {
            Ok(result) => result,
            Err(_) => continue,
        };

        for &bar_child in bar_children.iter() {
            let (mut bar_indicator_sprite, mut bar_indicator_transform) =
                match bar_child_indicator_query.get_mut(bar_child) {
                    Ok(result) => result,
                    Err(_) => continue,
                };

            let bar_width = bar_sprite.custom_size.unwrap().x;
            let bar_indicator_height = bar_indicator_sprite.custom_size.unwrap().y;

            let bar_indicator_width = (bar_width * progressive.get_progress()).round();
            bar_indicator_sprite.custom_size =
                Some(Vec2::new(bar_indicator_width, bar_indicator_height));
            bar_indicator_transform.translation.x = bar_width * -0.5 + bar_indicator_width / 2.0;
        }
    }
}

fn update_cast_bar_visibility_system(
    mut bar_query: Query<(&Children, &mut Visibility, &CastBar)>,
    mut bar_child_visibility_query: Query<&mut Visibility, Without<CastBar>>,
    cast_ability_query: Query<&CastAbility>,
) {
    for (bar_children, mut bar_visibility, bar) in bar_query.iter_mut() {
        let is_casting = match cast_ability_query.get(bar.entity()) {
            Ok(cast_ability) => {
                cast_ability.duration_timer.elapsed_secs() > 0.0
                    && !cast_ability.duration_timer.finished()
            }
            Err(_) => false,
        };

        bar_visibility.is_visible = is_casting;

        for &bar_child in bar_children.iter() {
            let mut bar_child_visibility = match bar_child_visibility_query.get_mut(bar_child) {
                Ok(result) => result,
                Err(_) => continue,
            };

            bar_child_visibility.is_visible = is_casting;
        }
    }
}

fn handle_player_target_changed_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_target_changed_event_reader: EventReader<PlayerTargetChanged>,
    player_target_indicator_query: Query<Entity, With<PlayerTargetIndicator>>,
) {
    let player_target_changed = match player_target_changed_event_reader.iter().last() {
        Some(result) => result,
        None => return,
    };

    for player_target_indicator_entity in player_target_indicator_query.iter() {
        commands.entity(player_target_indicator_entity).despawn();
    }

    let target_entity = match player_target_changed.target_entity {
        Some(result) => result,
        None => return,
    };

    let player_target_indicator_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.get_handle(crate::Sprite::SHEET_PATH),
            sprite: TextureAtlasSprite::new(crate::Sprite::TargetIndicator.index()),
            ..Default::default()
        })
        .insert(PlayerTargetIndicator)
        .id();

    commands
        .entity(target_entity)
        .add_child(player_target_indicator_entity);
}

fn spawn_bar<T: Component>(
    color: Color,
    translation: Vec3,
    size: Vec2,
    component: T,
    commands: &mut Commands,
    bar_text_font_handle: Option<Handle<Font>>,
) -> Entity {
    let mut bar_background_color = color;
    bar_background_color.set_a(BAR_BACKGROUND_COLOR_ALPHA);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                color: bar_background_color,
                ..Default::default()
            },
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(component)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(0.0, size.y)),
                    color,
                    ..Default::default()
                },
                ..Default::default()
            });

            if let Some(bar_text_font_handle) = bar_text_font_handle {
                let text_style = TextStyle {
                    font: bar_text_font_handle.clone(),
                    font_size: BAR_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                };
                let text_alignment = TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                };

                parent.spawn_bundle(Text2dBundle {
                    text: Text::with_section("", text_style, text_alignment),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        BAR_TEXT_VERTICAL_OFFSET,
                        1.0,
                    )),
                    ..Default::default()
                });
            }
        })
        .id()
}
