use super::easing::*;
use crate::{
    effect::{MomentaryEffectPerformed, PerformedMomentaryEffect},
    AppState,
};
use bevy::prelude::*;

const FONT_PATH: &str = "fonts/04b03.ttf";
const FONT_SIZE: f32 = 12.0;

const DAMAGE_COLOR: Color = Color::rgb(231.0 / 255.0, 39.0 / 255.0, 37.0 / 255.0);
const HEAL_COLOR: Color = Color::rgb(0.0, 231.0 / 255.0, 0.0);

const ANIMATION_TRANSLATION_Y: f32 = 6.0;
const ANIMATION_DURATION: f32 = 1.0;

#[derive(Component)]
struct FloatingText {
    animation_timer: Timer,
}

impl FloatingText {
    pub fn new() -> Self {
        Self {
            animation_timer: Timer::from_seconds(ANIMATION_DURATION, false),
        }
    }
}

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_system)
                .with_system(animate_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_system));
    }
}

fn spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut momentary_effect_performed_event_reader: EventReader<MomentaryEffectPerformed>,
    query: Query<&Transform>,
) {
    for momentary_effect_performed in momentary_effect_performed_event_reader.iter() {
        // TODO: Change animation based on critical.
        let (points, _, color) = match momentary_effect_performed.performed_momentary_effect {
            PerformedMomentaryEffect::Damage(points, is_critical) => {
                (points, is_critical, DAMAGE_COLOR)
            }
            PerformedMomentaryEffect::Heal(points, is_critical) => {
                (points, is_critical, HEAL_COLOR)
            }
        };

        let entity_transform = query.get(momentary_effect_performed.entity).unwrap();
        let translation =
            entity_transform.translation + Vec3::new(0.0, crate::Sprite::SIZE / 2.0 + 12.0, 0.0);
        let transform = Transform::from_translation(translation);

        let text_style = TextStyle {
            font: asset_server.load(FONT_PATH),
            font_size: FONT_SIZE,
            color,
        };
        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(points.to_string(), text_style, text_alignment),
                transform,
                ..Default::default()
            })
            .insert(FloatingText::new());
    }
}

fn animate_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Text, &mut FloatingText)>,
) {
    for (entity, mut transform, mut text, mut floating_text) in query.iter_mut() {
        let previous_translation_y = ANIMATION_TRANSLATION_Y
            * ease(floating_text.animation_timer.percent(), Easing::OutQuart);
        floating_text.animation_timer.tick(time.delta());

        if floating_text.animation_timer.finished() {
            commands.entity(entity).despawn();
        } else {
            let translation_y = ANIMATION_TRANSLATION_Y
                * ease(floating_text.animation_timer.percent(), Easing::OutQuart);
            transform.translation.y += translation_y - previous_translation_y;

            if floating_text.animation_timer.percent() > 0.5 {
                let color_alpha = 1.0 - (floating_text.animation_timer.percent() - 0.5) * 2.0;
                text.sections[0].style.color.set_a(color_alpha);
            }
        }
    }
}

fn despawn_system(mut commands: Commands, query: Query<Entity, With<FloatingText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
