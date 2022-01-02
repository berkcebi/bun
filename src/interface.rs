use crate::{
    ability::CastAbility, health::Health, mana::Mana, player::Player, CAMERA_SCALE, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use bevy::{ecs::component::Component, prelude::*};

const WIDTH: f32 = WINDOW_WIDTH * CAMERA_SCALE;
const HEIGHT: f32 = WINDOW_HEIGHT * CAMERA_SCALE;

const TRANSLATION_Z: f32 = 50.0;

const BAR_WIDTH_SMALL: f32 = 96.0;
const BAR_WIDTH_LARGE: f32 = 144.0;
const BAR_HEIGHT: f32 = 16.0;
const BAR_MARGIN: f32 = 16.0;
const BAR_BACKGROUND_COLOR_ALPHA: f32 = 0.25;
const BAR_TEXT_FONT_PATH: &str = "fonts/04b03.ttf";
const BAR_TEXT_FONT_SIZE: f32 = 12.0;
const BAR_TEXT_VERTICAL_OFFSET: f32 = -0.5;

const HEALTH_BAR_COLOR: Color = Color::rgb(231.0 / 255.0, 39.0 / 255.0, 37.0 / 255.0);
const MANA_BAR_COLOR: Color = Color::rgb(43.0 / 255.0, 102.0 / 255.0, 201.0 / 255.0);
const CAST_BAR_COLOR: Color = Color::rgb(1.0, 240.0 / 255.0, 0.0);

trait Progressive: Component {
    fn get_progress(&self) -> f32;
    fn get_description(&self) -> String;
}

impl Progressive for Health {
    fn get_progress(&self) -> f32 {
        self.points as f32 / self.max_points as f32
    }

    fn get_description(&self) -> String {
        format!("{}/{}", self.points, self.max_points)
    }
}

impl Progressive for Mana {
    fn get_progress(&self) -> f32 {
        self.points as f32 / self.max_points as f32
    }

    fn get_description(&self) -> String {
        format!("{}/{}", self.points, self.max_points)
    }
}

impl Progressive for CastAbility {
    fn get_progress(&self) -> f32 {
        self.duration_timer.percent()
    }

    fn get_description(&self) -> String {
        self.ability.name.to_string()
    }
}

struct HealthBar;
struct ManaBar;
struct CastBar;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(update_bar_text::<Health, HealthBar>.system())
            .add_system(update_bar_indicator::<Health, HealthBar>.system())
            .add_system(update_bar_text::<Mana, ManaBar>.system())
            .add_system(update_bar_indicator::<Mana, ManaBar>.system())
            .add_system(update_bar_text::<CastAbility, CastBar>.system())
            .add_system(update_bar_indicator::<CastAbility, CastBar>.system())
            .add_system(update_cast_bar_visible.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let bar_text_font = asset_server.load(BAR_TEXT_FONT_PATH);

    spawn_bar(
        HEALTH_BAR_COLOR,
        Vec3::new(
            WIDTH * -0.5 + BAR_WIDTH_SMALL * 0.5 + BAR_MARGIN,
            HEIGHT * 0.5 - BAR_HEIGHT * 0.5 - BAR_MARGIN,
            TRANSLATION_Z,
        ),
        BAR_WIDTH_SMALL,
        HealthBar,
        &mut commands,
        bar_text_font.clone(),
        &mut color_materials,
    );

    spawn_bar(
        MANA_BAR_COLOR,
        Vec3::new(
            WIDTH * -0.5 + BAR_WIDTH_SMALL * 0.5 + BAR_MARGIN,
            HEIGHT * 0.5 - BAR_HEIGHT * 1.5 - BAR_MARGIN * 1.5,
            TRANSLATION_Z,
        ),
        BAR_WIDTH_SMALL,
        ManaBar,
        &mut commands,
        bar_text_font.clone(),
        &mut color_materials,
    );

    spawn_bar(
        CAST_BAR_COLOR,
        Vec3::new(0.0, HEIGHT / -4.0, TRANSLATION_Z),
        BAR_WIDTH_LARGE,
        CastBar,
        &mut commands,
        bar_text_font,
        &mut color_materials,
    );
}

fn update_bar_text<T: Progressive, U: Component>(
    bar_children_query: Query<&Children, With<U>>,
    mut bar_children_text_query: Query<&mut Text>,
    progressive_query: Query<&T, With<Player>>,
) {
    let progressive = match progressive_query.single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let bar_children = bar_children_query.single().unwrap();
    for &bar_child in bar_children.iter() {
        let mut bar_text = match bar_children_text_query.get_mut(bar_child) {
            Ok(result) => result,
            Err(_) => continue,
        };

        bar_text.sections[0].value = progressive.get_description();
    }
}

fn update_bar_indicator<T: Progressive, U: Component>(
    bar_query: Query<(&Children, &Sprite), With<U>>,
    mut bar_children_indicator_query: Query<(&mut Sprite, &mut Transform), Without<U>>,
    progressive_query: Query<&T, With<Player>>,
) {
    let progressive = match progressive_query.single() {
        Ok(result) => result,
        Err(_) => return,
    };

    let (bar_children, bar_sprite) = bar_query.single().unwrap();
    for &bar_child in bar_children.iter() {
        let (mut bar_indicator_sprite, mut bar_indicator_transform) =
            match bar_children_indicator_query.get_mut(bar_child) {
                Ok(result) => result,
                Err(_) => continue,
            };

        let bar_width = bar_sprite.size.x;

        let bar_indicator_width = (bar_width * progressive.get_progress()).round();
        bar_indicator_sprite.size.x = bar_indicator_width;
        bar_indicator_transform.translation.x = bar_width * -0.5 + bar_indicator_width / 2.0;
    }
}

fn update_cast_bar_visible(
    mut bar_query: Query<(&Children, &mut Visible), With<CastBar>>,
    mut bar_children_visible_query: Query<&mut Visible, Without<CastBar>>,
    cast_ability_query: Query<&CastAbility, With<Player>>,
) {
    let is_visible = match cast_ability_query.single() {
        Ok(cast_ability) => {
            cast_ability.duration_timer.elapsed_secs() > 0.0
                && !cast_ability.duration_timer.finished()
        }
        Err(_) => false,
    };

    let (bar_children, mut bar_visible) = bar_query.single_mut().unwrap();

    bar_visible.is_visible = is_visible;

    for &bar_child in bar_children.iter() {
        let mut bar_visible = match bar_children_visible_query.get_mut(bar_child) {
            Ok(result) => result,
            Err(_) => continue,
        };

        bar_visible.is_visible = is_visible;
    }
}

fn spawn_bar<T: Component>(
    color: Color,
    translation: Vec3,
    width: f32,
    component: T,
    commands: &mut Commands,
    bar_text_font_handle: Handle<Font>,
    color_materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut bar_background_color = color;
    bar_background_color.set_a(BAR_BACKGROUND_COLOR_ALPHA);

    commands
        .spawn_bundle(SpriteBundle {
            material: color_materials.add(bar_background_color.into()),
            sprite: Sprite::new(Vec2::new(width, BAR_HEIGHT)),
            transform: Transform::from_translation(translation),
            ..Default::default()
        })
        .insert(component)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: color_materials.add(color.into()),
                sprite: Sprite::new(Vec2::new(0.0, BAR_HEIGHT)),
                ..Default::default()
            });

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
        });
}
