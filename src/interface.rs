use crate::{health::Health, mana::Mana, player::Player, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::{ecs::component::Component, prelude::*};

const CAMERA_SCALE: f32 = 1.0 / 2.0;

const WIDTH: f32 = WINDOW_WIDTH * CAMERA_SCALE;
const HEIGHT: f32 = WINDOW_HEIGHT * CAMERA_SCALE;

const BAR_WIDTH: f32 = 96.0;
const BAR_HEIGHT: f32 = 16.0;
const BAR_MARGIN: f32 = 16.0;
const BAR_BACKGROUND_COLOR_ALPHA: f32 = 0.25;
const BAR_TEXT_FONT_PATH: &str = "fonts/04b03.ttf";
const BAR_TEXT_FONT_SIZE: f32 = 12.0;
const BAR_TEXT_VERTICAL_OFFSET: f32 = -0.5;

const HEALTH_BAR_COLOR: Color = Color::rgb(231.0 / 255.0, 39.0 / 255.0, 37.0 / 255.0);
const MANA_BAR_COLOR: Color = Color::rgb(43.0 / 255.0, 102.0 / 255.0, 201.0 / 255.0);

trait BarComponent: Component {
    fn get_value(&self) -> u8;
    fn get_max_value(&self) -> u8;
}

impl BarComponent for Health {
    fn get_value(&self) -> u8 {
        self.points
    }

    fn get_max_value(&self) -> u8 {
        self.max_points
    }
}

impl BarComponent for Mana {
    fn get_value(&self) -> u8 {
        self.points
    }

    fn get_max_value(&self) -> u8 {
        self.max_points
    }
}

struct HealthBarIndicator;
struct HealthBarText;
struct ManaBarIndicator;
struct ManaBarText;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_bar_text::<Health, HealthBarText>.system())
            .add_system(update_bar_indicator::<Health, HealthBarIndicator>.system())
            .add_system(update_bar_text::<Mana, ManaBarText>.system())
            .add_system(update_bar_indicator::<Mana, ManaBarIndicator>.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let bar_text_font = asset_server.load(BAR_TEXT_FONT_PATH);

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = CAMERA_SCALE;
    commands.spawn_bundle(camera_bundle);

    spawn_bar(
        HEALTH_BAR_COLOR,
        0.0,
        HealthBarIndicator,
        HealthBarText,
        &mut commands,
        bar_text_font.clone(),
        &mut color_materials,
    );

    spawn_bar(
        MANA_BAR_COLOR,
        BAR_MARGIN * -0.5 - BAR_HEIGHT,
        ManaBarIndicator,
        ManaBarText,
        &mut commands,
        bar_text_font.clone(),
        &mut color_materials,
    );
}

fn update_bar_text<T: BarComponent, U: Component>(
    mut bar_text_query: Query<&mut Text, With<U>>,
    component_query: Query<&T, With<Player>>,
) {
    let mut bar_text = bar_text_query.single_mut().unwrap();
    let component = component_query.single().unwrap();

    bar_text.sections[0].value = format!("{}/{}", component.get_value(), component.get_max_value());
}

fn update_bar_indicator<T: BarComponent, U: Component>(
    mut bar_indicator_query: Query<(&mut Sprite, &mut Transform), With<U>>,
    component_query: Query<&T, With<Player>>,
) {
    let (mut bar_indicator_sprite, mut bar_indicator_transform) =
        bar_indicator_query.single_mut().unwrap();
    let component = component_query.single().unwrap();

    let bar_indicator_width =
        (BAR_WIDTH * component.get_value() as f32 / component.get_max_value() as f32).floor();
    bar_indicator_sprite.size.x = bar_indicator_width;
    bar_indicator_transform.translation.x = BAR_WIDTH / 2.0 * -1.0 + bar_indicator_width / 2.0;
}

fn spawn_bar<T: Component, U: Component>(
    color: Color,
    y_offset: f32,
    indicator_component: T,
    text_component: U,
    commands: &mut Commands,
    bar_text_font_handle: Handle<Font>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut bar_background_color = color.clone();
    bar_background_color.set_a(BAR_BACKGROUND_COLOR_ALPHA);

    commands
        .spawn_bundle(SpriteBundle {
            material: color_materials.add(bar_background_color.into()),
            sprite: Sprite::new(Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(
                -1.0 * WIDTH / 2.0 + BAR_WIDTH / 2.0 + BAR_MARGIN,
                HEIGHT / 2.0 - BAR_HEIGHT / 2.0 - BAR_MARGIN + y_offset,
                0.0,
            )),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    material: color_materials.add(color.into()),
                    sprite: Sprite::new(Vec2::new(0.0, BAR_HEIGHT)),
                    ..Default::default()
                })
                .insert(indicator_component);

            let text_style = TextStyle {
                font: bar_text_font_handle.clone(),
                font_size: BAR_TEXT_FONT_SIZE,
                color: Color::WHITE,
            };
            let text_alignment = TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            };

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section("", text_style.clone(), text_alignment),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        BAR_TEXT_VERTICAL_OFFSET,
                        1.0,
                    )),
                    ..Default::default()
                })
                .insert(text_component);
        });
}
