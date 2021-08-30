use crate::{mana::Mana, player::Player, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

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

const MANA_BAR_COLOR: Color = Color::rgb(43.0 / 255.0, 102.0 / 255.0, 201.0 / 255.0);

struct ManaBarIndicator;
struct ManaBarText;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_mana_bar_text.system())
            .add_system(update_mana_bar_indicator.system());
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

    let mut mana_bar_background_color = MANA_BAR_COLOR.clone();
    mana_bar_background_color.set_a(BAR_BACKGROUND_COLOR_ALPHA);

    commands
        .spawn_bundle(SpriteBundle {
            material: color_materials.add(mana_bar_background_color.into()),
            sprite: Sprite::new(Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(
                -1.0 * WIDTH / 2.0 + BAR_WIDTH / 2.0 + BAR_MARGIN,
                HEIGHT / 2.0 - BAR_HEIGHT / 2.0 - BAR_MARGIN,
                0.0,
            )),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    material: color_materials.add(MANA_BAR_COLOR.into()),
                    sprite: Sprite::new(Vec2::new(0.0, BAR_HEIGHT)),
                    ..Default::default()
                })
                .insert(ManaBarIndicator);

            let text_style = TextStyle {
                font: bar_text_font,
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
                .insert(ManaBarText);
        });
}

fn update_mana_bar_text(
    mut mana_bar_text_query: Query<&mut Text, With<ManaBarText>>,
    mana_query: Query<&Mana, With<Player>>,
) {
    let mut mana_bar_text = mana_bar_text_query.single_mut().unwrap();
    let mana = mana_query.single().unwrap();

    mana_bar_text.sections[0].value = format!("{}/{}", mana.points, mana.max_points);
}

fn update_mana_bar_indicator(
    mut mana_bar_indicator_query: Query<(&mut Sprite, &mut Transform), With<ManaBarIndicator>>,
    mana_query: Query<&Mana, With<Player>>,
) {
    let (mut mana_bar_indicator_sprite, mut mana_bar_indicator_transform) =
        mana_bar_indicator_query.single_mut().unwrap();
    let mana = mana_query.single().unwrap();

    let mana_bar_indicator_width =
        (BAR_WIDTH * mana.points as f32 / mana.max_points as f32).floor();
    mana_bar_indicator_sprite.size.x = mana_bar_indicator_width;
    mana_bar_indicator_transform.translation.x =
        BAR_WIDTH / 2.0 * -1.0 + mana_bar_indicator_width / 2.0;
}
