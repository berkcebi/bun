use super::easing::*;
use crate::{
    effect::{MomentaryEffectPerformed, PerformedMomentaryEffect},
    sprite::Sprite,
    AppState,
};
use bevy::prelude::*;
use std::collections::HashMap;

const FONT_PATH: &str = "fonts/04b03.ttf";
const FONT_SIZE: f32 = 12.0;

const DAMAGE_COLOR: Color = Color::rgb(231.0 / 255.0, 39.0 / 255.0, 37.0 / 255.0);
const HEAL_COLOR: Color = Color::rgb(0.0, 231.0 / 255.0, 0.0);

const TRANSLATION_Y: f32 = 12.0;
const TRANSLATION_X: [f32; 3] = [0.0, 18.0, -18.0];

const ANIMATION_TRANSLATION_Y: f32 = 6.0;
const ANIMATION_DURATION: f32 = 1.0;

#[derive(Component)]
struct FloatingText {
    entity: Entity,
    animation_timer: Timer,
}

impl FloatingText {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
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
) {
    for momentary_effect_performed in momentary_effect_performed_event_reader.iter() {
        let (points, _, color) = match momentary_effect_performed.performed_momentary_effect {
            PerformedMomentaryEffect::Damage(points, is_critical) => {
                (points, is_critical, DAMAGE_COLOR)
            }
            PerformedMomentaryEffect::Heal(points, is_critical) => {
                (points, is_critical, HEAL_COLOR)
            }
        };

        let text_style = TextStyle {
            font: asset_server.load(FONT_PATH),
            font_size: FONT_SIZE,
            color,
        };

        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(points.to_string(), text_style)
                    .with_alignment(TextAlignment::CENTER),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(FloatingText::new(momentary_effect_performed.entity));
    }
}

fn animate_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Visibility,
        &mut Text,
        &mut FloatingText,
    )>,
    entity_query: Query<&Transform, Without<FloatingText>>,
) {
    let mut index_by_entity = HashMap::new();
    for (floating_text_entity, mut transform, mut visibility, mut text, mut floating_text) in
        query.iter_mut()
    {
        floating_text.animation_timer.tick(time.delta());
        if floating_text.animation_timer.finished() {
            commands.entity(floating_text_entity).despawn();

            continue;
        }

        let index = index_by_entity.entry(floating_text.entity).or_insert(0);

        if !visibility.is_visible {
            visibility.is_visible = true;
        }

        // TODO: Change animation based on critical.

        let entity_transform = entity_query.get(floating_text.entity).unwrap();

        transform.translation = entity_transform.translation
            + Vec3::new(
                TRANSLATION_X[*index % TRANSLATION_X.len()],
                Sprite::SIZE / 2.0
                    + TRANSLATION_Y
                    + ANIMATION_TRANSLATION_Y
                        * ease(floating_text.animation_timer.percent(), Easing::OutQuart),
                0.0,
            );

        if floating_text.animation_timer.percent() > 0.5 {
            let color_alpha = 1.0 - (floating_text.animation_timer.percent() - 0.5) * 2.0;
            text.sections[0].style.color.set_a(color_alpha);
        }

        *index += 1;
    }
}

fn despawn_system(mut commands: Commands, query: Query<Entity, With<FloatingText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
