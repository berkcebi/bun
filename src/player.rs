use crate::{
    ability::{Ability, TryAbility},
    creature::Creature,
    effect::{Effect, LastingEffect, MomentaryEffect, MomentaryEffectSchedule},
    position::ChangePosition,
    target::Target,
    AppState, CAMERA_SCALE, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use bevy::prelude::*;

const DISTANCE_LIMIT: f32 = 40.0;

/// Event to communicate player target changing.
pub struct PlayerTargetChanged {
    pub target_entity: Option<Entity>,
}

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerTargetChanged>().add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(handle_keyboard_input_system)
                .with_system(handle_cursor_moved_system),
        );
    }
}

fn handle_keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut change_position_event_writer: EventWriter<ChangePosition>,
    mut try_ability_event_writer: EventWriter<TryAbility>,
    query: Query<(Entity, &Target), With<Player>>,
) {
    let (entity, target) = query.single();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }

    if direction != Vec3::ZERO {
        change_position_event_writer.send(ChangePosition { entity, direction });
    }

    if keyboard_input.just_pressed(KeyCode::Key1) {
        try_ability_event_writer.send(TryAbility {
            source: entity,
            ability: Ability {
                name: "Fireball",
                mana_points: 20,
                cast_duration: 2.5,
                range: 200.0,
                effect: Effect::Momentary(
                    MomentaryEffect::Damage(30, 50),
                    MomentaryEffectSchedule::Once,
                ),
                secondary_effect: Some(Effect::Momentary(
                    MomentaryEffect::Damage(2, 3),
                    MomentaryEffectSchedule::Periodic(3.0, 12.0),
                )),
            },
            target: target.entity,
        });
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        try_ability_event_writer.send(TryAbility {
            source: entity,
            ability: Ability {
                name: "Fire Blast",
                mana_points: 15,
                cast_duration: 0.0,
                range: 125.0,
                effect: Effect::Momentary(
                    MomentaryEffect::Damage(20, 30),
                    MomentaryEffectSchedule::Once,
                ),
                secondary_effect: None,
            },
            target: target.entity,
        });
    }

    if keyboard_input.just_pressed(KeyCode::Key3) {
        try_ability_event_writer.send(TryAbility {
            source: entity,
            ability: Ability {
                name: "Lesser Heal",
                mana_points: 15,
                cast_duration: 1.5,
                range: 200.0,
                effect: Effect::Momentary(
                    MomentaryEffect::Heal(40, 60),
                    MomentaryEffectSchedule::Once,
                ),
                secondary_effect: None,
            },
            target: target.entity,
        });
    }

    if keyboard_input.just_pressed(KeyCode::Q) {
        try_ability_event_writer.send(TryAbility {
            source: entity,
            ability: Ability {
                name: "Silence",
                mana_points: 20,
                cast_duration: 0.0,
                range: 200.0,
                effect: Effect::Lasting(LastingEffect::Silence, 4.0),
                secondary_effect: None,
            },
            target: target.entity,
        });
    }
}

fn handle_cursor_moved_system(
    mut cursor_moved_event_reader: EventReader<CursorMoved>,
    mut player_target_changed_event_writer: EventWriter<PlayerTargetChanged>,
    creature_query: Query<(Entity, &Transform), With<Creature>>,
    mut player_query: Query<&mut Target, With<Player>>,
) {
    let cursor_moved = match cursor_moved_event_reader.iter().last() {
        Some(result) => result,
        None => return,
    };

    let cursor_position = cursor_moved.position;
    let cursor_position_matrix = cursor_position.extend(0.0).extend(1.0);

    // TODO: Calcuate based on camera's actual transform, in case it's transformed down the line.
    let camera_transform = Transform::default()
        .with_translation(Vec3::new(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0) / -2.0 * CAMERA_SCALE)
        .with_scale(Vec2::splat(CAMERA_SCALE).extend(1.0));
    let adjusted_cursor_position_matrix =
        camera_transform.compute_matrix() * cursor_position_matrix;
    let adjusted_cursor_position = adjusted_cursor_position_matrix.truncate().truncate();

    let closest_creature_entity = creature_query
        .iter()
        .fold(None, |closest_entity, (entity, transform)| {
            let position = transform.translation.truncate();
            let distance = position.distance(adjusted_cursor_position);

            match closest_entity {
                None if distance < DISTANCE_LIMIT => Some((entity, distance)),
                Some((_, previous_distance)) if distance < previous_distance => {
                    Some((entity, distance))
                }
                _ => closest_entity,
            }
        })
        .map(|(entity, _)| entity);

    let mut player_target = player_query.single_mut();
    if player_target.entity != closest_creature_entity {
        player_target.entity = closest_creature_entity;

        player_target_changed_event_writer.send(PlayerTargetChanged {
            target_entity: player_target.entity,
        });
    }
}
