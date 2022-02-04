use super::{HEIGHT, TRANSLATION_Z, WIDTH};
use crate::{
    ability::CastAbility, enemy::Enemy, health::Health, mana::Mana, player::Player, AppState,
};
use bevy::{ecs::component::Component, prelude::*};

const PLAYER_WIDTH: f32 = 96.0;
const PLAYER_CAST_WIDTH: f32 = 144.0;
const PLAYER_HEIGHT: f32 = 16.0;
const ENEMY_HEIGHT: f32 = 4.0;
const PLAYER_MARGIN: f32 = 16.0;
const ENEMY_MARGIN: f32 = 1.0;
const TEXT_VERTICAL_OFFSET: f32 = -0.5;

const FONT_PATH: &str = "fonts/04b03.ttf";
const FONT_SIZE: f32 = 12.0;

const HEALTH_COLOR: Color = Color::rgb(231.0 / 255.0, 39.0 / 255.0, 37.0 / 255.0);
const MANA_COLOR: Color = Color::rgb(43.0 / 255.0, 102.0 / 255.0, 201.0 / 255.0);
const CAST_COLOR: Color = Color::rgb(1.0, 240.0 / 255.0, 0.0);
const BACKGROUND_COLOR_ALPHA: f32 = 0.25;

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

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_player_system)
                .with_system(spawn_enemy_system)
                .with_system(update_text_system::<HealthBar>)
                .with_system(update_indicator_system::<HealthBar>)
                .with_system(update_text_system::<ManaBar>)
                .with_system(update_indicator_system::<ManaBar>)
                .with_system(update_text_system::<CastBar>)
                .with_system(update_indicator_system::<CastBar>)
                .with_system(update_cast_visibility_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_system));
    }
}

fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Added<Player>>,
) {
    let entity = match query.get_single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let font = asset_server.load(FONT_PATH);

    spawn(
        HEALTH_COLOR,
        Vec3::new(
            WIDTH * -0.5 + PLAYER_WIDTH * 0.5 + PLAYER_MARGIN,
            HEIGHT * 0.5 - PLAYER_HEIGHT * 0.5 - PLAYER_MARGIN,
            TRANSLATION_Z,
        ),
        Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        true,
        HealthBar { entity },
        &mut commands,
        Some(font.clone()),
    );

    spawn(
        MANA_COLOR,
        Vec3::new(
            WIDTH * -0.5 + PLAYER_WIDTH * 0.5 + PLAYER_MARGIN,
            HEIGHT * 0.5 - PLAYER_HEIGHT * 1.5 - PLAYER_MARGIN * 1.5,
            TRANSLATION_Z,
        ),
        Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        true,
        ManaBar { entity },
        &mut commands,
        Some(font.clone()),
    );

    spawn(
        CAST_COLOR,
        Vec3::new(0.0, HEIGHT / -4.0, TRANSLATION_Z),
        Vec2::new(PLAYER_CAST_WIDTH, PLAYER_HEIGHT),
        false,
        CastBar { entity },
        &mut commands,
        Some(font),
    );
}

fn spawn_enemy_system(mut commands: Commands, query: Query<Entity, Added<Enemy>>) {
    for entity in query.iter() {
        let health_entity = spawn(
            HEALTH_COLOR,
            Vec3::new(
                0.0,
                (crate::Sprite::SIZE + ENEMY_HEIGHT) / 2.0 + ENEMY_MARGIN,
                TRANSLATION_Z,
            ),
            Vec2::new(crate::Sprite::SIZE, ENEMY_HEIGHT),
            true,
            HealthBar { entity },
            &mut commands,
            None,
        );

        commands.entity(entity).add_child(health_entity);
    }
}

fn update_text_system<T: Bar>(
    query: Query<(&Children, &T)>,
    mut child_text_query: Query<&mut Text>,
    progressive_query: Query<&T::Type>,
) {
    for (children, bar) in query.iter() {
        let progressive = match progressive_query.get(bar.entity()) {
            Ok(result) => result,
            Err(_) => continue,
        };

        for &child in children.iter() {
            let mut text = match child_text_query.get_mut(child) {
                Ok(result) => result,
                Err(_) => continue,
            };

            text.sections[0].value = progressive.get_progress_description();
        }
    }
}

fn update_indicator_system<T: Bar>(
    query: Query<(&Children, &Sprite, &T)>,
    mut child_indicator_query: Query<(&mut Sprite, &mut Transform), Without<T>>,
    progressive_query: Query<&T::Type>,
) {
    for (children, sprite, bar) in query.iter() {
        let progressive = match progressive_query.get(bar.entity()) {
            Ok(result) => result,
            Err(_) => continue,
        };

        for &child in children.iter() {
            let (mut indicator_sprite, mut indicator_transform) =
                match child_indicator_query.get_mut(child) {
                    Ok(result) => result,
                    Err(_) => continue,
                };

            let width = sprite.custom_size.unwrap().x;
            let indicator_height = indicator_sprite.custom_size.unwrap().y;

            let indicator_width = (width * progressive.get_progress()).round();
            indicator_sprite.custom_size = Some(Vec2::new(indicator_width, indicator_height));
            indicator_transform.translation.x = width * -0.5 + indicator_width / 2.0;
        }
    }
}

fn update_cast_visibility_system(
    mut query: Query<(&Children, &mut Visibility, &CastBar)>,
    mut child_visibility_query: Query<&mut Visibility, Without<CastBar>>,
    cast_ability_query: Query<&CastAbility>,
) {
    for (children, mut visibility, bar) in query.iter_mut() {
        let is_casting = match cast_ability_query.get(bar.entity()) {
            Ok(cast_ability) => {
                cast_ability.duration_timer.elapsed_secs() > 0.0
                    && !cast_ability.duration_timer.finished()
            }
            Err(_) => false,
        };

        visibility.is_visible = is_casting;

        for &child in children.iter() {
            let mut child_visibility = match child_visibility_query.get_mut(child) {
                Ok(result) => result,
                Err(_) => continue,
            };

            child_visibility.is_visible = is_casting;
        }
    }
}

fn despawn_system(
    mut commands: Commands,
    health_query: Query<Entity, With<HealthBar>>,
    mana_query: Query<Entity, With<ManaBar>>,
    cast_query: Query<Entity, With<CastBar>>,
) {
    for entity in health_query
        .iter()
        .chain(mana_query.iter())
        .chain(cast_query.iter())
    {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn<T: Component>(
    color: Color,
    translation: Vec3,
    size: Vec2,
    is_visible: bool,
    component: T,
    commands: &mut Commands,
    font_handle: Option<Handle<Font>>,
) -> Entity {
    let mut background_color = color;
    background_color.set_a(BACKGROUND_COLOR_ALPHA);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(size),
                color: background_color,
                ..Default::default()
            },
            transform: Transform::from_translation(translation),
            visibility: Visibility { is_visible },
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

            if let Some(font_handle) = font_handle {
                let text_style = TextStyle {
                    font: font_handle,
                    font_size: FONT_SIZE,
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
                        TEXT_VERTICAL_OFFSET,
                        1.0,
                    )),
                    ..Default::default()
                });
            }
        })
        .id()
}
