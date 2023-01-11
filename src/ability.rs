use crate::{
    creature::Creature,
    effect::{Effect, LastingEffect, LastingEffects, PerformEffect},
    intersect_line_aabb::is_intersecting,
    level::Obstacle,
    mana::{Mana, RegenManaCooldown},
    position::ChangingPosition,
    AppState,
};
use bevy::prelude::*;
use std::collections::HashMap;

const ABILITY_GLOBAL_COOLDOWN_DURATION: f32 = 1.5;

#[derive(Clone, Copy)]
pub struct Ability {
    pub id: u8,
    pub name: &'static str,
    pub mana_points: u16,
    pub cast_duration: f32,
    pub cooldown_duration: f32,
    pub range: f32,
    pub effect: (Effect, AbilityTargetMode),
    pub secondary_effect: Option<(Effect, AbilityTargetMode)>,
}

impl Ability {
    fn requires_target(&self) -> bool {
        let (_, target_mode) = self.effect;
        let effect_requires_target = target_mode == AbilityTargetMode::Single;

        let secondary_effect_requires_target = match self.secondary_effect {
            Some((_, secondary_target_mode)) => secondary_target_mode == AbilityTargetMode::Single,
            None => false,
        };

        effect_requires_target || secondary_effect_requires_target
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AbilityTargetMode {
    Single,
    Area,
}

/// Event to initiate an ability, if possible.
pub struct TryAbility {
    pub source: Entity,
    pub ability: Ability,
    pub target: Option<Entity>,
}

/// Event to cancel casting ability.
pub struct CancelCastAbility {
    pub source: Entity,
}

/// Internal event to perform an ability via a try ability event.
struct PerformAbility {
    source: Entity,
    ability: Ability,
    target: Option<Entity>,
}

/// Component to store cast duration for an ability.
#[derive(Component)]
pub struct CastAbility {
    pub ability: Ability,
    pub target: Option<Entity>,
    pub duration_timer: Timer,
}

impl CastAbility {
    pub fn new(ability: Ability, target: Option<Entity>) -> Self {
        Self {
            ability,
            target,
            duration_timer: Timer::from_seconds(ability.cast_duration, TimerMode::Once),
        }
    }
}

/// Component to disable all abilities for a duration after using any ability.
#[derive(Component)]
struct AbilityGlobalCooldown {
    duration_timer: Timer,
}

impl Default for AbilityGlobalCooldown {
    fn default() -> Self {
        Self {
            duration_timer: Timer::from_seconds(ABILITY_GLOBAL_COOLDOWN_DURATION, TimerMode::Once),
        }
    }
}

/// Component to disable abilities for a duration.
#[derive(Component, Default)]
pub struct AbilityCooldowns {
    instances_by_id: HashMap<u8, AbilityCooldownInstance>,
}

impl AbilityCooldowns {
    fn push(&mut self, ability: Ability) {
        self.instances_by_id
            .insert(ability.id, AbilityCooldownInstance::new(ability));
    }
}

struct AbilityCooldownInstance {
    duration_timer: Timer,
}

impl AbilityCooldownInstance {
    fn new(ability: Ability) -> Self {
        assert!(ability.cooldown_duration > 0.0);

        Self {
            duration_timer: Timer::from_seconds(ability.cooldown_duration, TimerMode::Once),
        }
    }
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TryAbility>()
            .add_event::<CancelCastAbility>()
            .add_event::<PerformAbility>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(remove_ability_global_cooldown_system)
                    .with_system(remove_ability_cooldowns_system)
                    .with_system(try_ability_system)
                    .with_system(cast_ability_system)
                    .with_system(cancel_cast_ability_system)
                    .with_system(perform_ability_system),
            );
    }
}

fn remove_ability_global_cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AbilityGlobalCooldown)>,
) {
    for (entity, mut ability_global_cooldown) in query.iter_mut() {
        ability_global_cooldown.duration_timer.tick(time.delta());

        if ability_global_cooldown.duration_timer.finished() {
            info!("Global cooldown over.");
            commands.entity(entity).remove::<AbilityGlobalCooldown>();
        }
    }
}

fn remove_ability_cooldowns_system(time: Res<Time>, mut query: Query<&mut AbilityCooldowns>) {
    for mut ability_cooldowns in query.iter_mut() {
        ability_cooldowns
            .instances_by_id
            .retain(|ability_id, ability_cooldown_instance| {
                ability_cooldown_instance.duration_timer.tick(time.delta());
                if ability_cooldown_instance.duration_timer.finished() {
                    info!("Ability ({ability_id}) cooldown over.");
                }

                !ability_cooldown_instance.duration_timer.finished()
            });
    }
}

fn try_ability_system(
    mut commands: Commands,
    mut try_ability_event_reader: EventReader<TryAbility>,
    mut perform_ability_event_writer: EventWriter<PerformAbility>,
    mut query: Query<(
        &Mana,
        &AbilityCooldowns,
        &LastingEffects,
        &Transform,
        Option<&CastAbility>,
        Option<&AbilityGlobalCooldown>,
        Option<&ChangingPosition>,
    )>,
    target_query: Query<&Transform>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
) {
    for try_ability in try_ability_event_reader.iter() {
        let (
            mana,
            ability_cooldowns,
            lasting_effects,
            transform,
            cast_ability,
            ability_global_cooldown,
            changing_position,
        ) = query.get_mut(try_ability.source).unwrap();

        if cast_ability.is_some() {
            info!("Casting another ability.");

            continue;
        }

        if lasting_effects
            .instances
            .iter()
            .any(|instance| matches!(instance.effect, LastingEffect::Silence))
        {
            info!("Silenced.");

            continue;
        }

        if changing_position.is_some() && try_ability.ability.cast_duration > 0.0 {
            info!("Moving.");

            continue;
        }

        if ability_cooldowns
            .instances_by_id
            .contains_key(&try_ability.ability.id)
        {
            info!("Ability in cooldown.");

            continue;
        }

        if ability_global_cooldown.is_some() {
            info!("In global cooldown.");

            continue;
        }

        if try_ability.ability.mana_points > mana.points {
            info!("Not enough mana.");

            continue;
        }

        if try_ability.ability.requires_target() {
            let target = match try_ability.target {
                Some(result) => result,
                None => {
                    info!("No target.");

                    continue;
                }
            };

            if target != try_ability.source {
                let position = transform.translation.truncate();

                let target_transform = target_query.get(target).unwrap();
                let target_position = target_transform.translation.truncate();

                match verify_target_position(
                    position,
                    target_position,
                    try_ability.ability.range,
                    &obstacle_query,
                ) {
                    Err(TargetPositionError::Range) => {
                        info!("Out of range.");

                        continue;
                    }
                    Err(TargetPositionError::Sight) => {
                        info!("Not in line of sight.");

                        continue;
                    }
                    Ok(_) => (),
                }
            }
        }

        commands
            .entity(try_ability.source)
            .insert(AbilityGlobalCooldown::default());

        if try_ability.ability.cast_duration > 0.0 {
            commands
                .entity(try_ability.source)
                .insert(CastAbility::new(try_ability.ability, try_ability.target));
        } else {
            perform_ability_event_writer.send(PerformAbility {
                source: try_ability.source,
                ability: try_ability.ability,
                target: try_ability.target,
            });
        }
    }
}

fn cast_ability_system(
    mut commands: Commands,
    time: Res<Time>,
    mut perform_ability_event_writer: EventWriter<PerformAbility>,
    mut query: Query<(Entity, &mut CastAbility, Option<&ChangingPosition>)>,
) {
    for (entity, mut cast_ability, changing_position) in query.iter_mut() {
        if changing_position.is_some() {
            commands.entity(entity).remove::<CastAbility>();

            continue;
        }

        cast_ability.duration_timer.tick(time.delta());

        if cast_ability.duration_timer.finished() {
            commands.entity(entity).remove::<CastAbility>();

            perform_ability_event_writer.send(PerformAbility {
                source: entity,
                ability: cast_ability.ability,
                target: cast_ability.target,
            });
        }
    }
}

fn cancel_cast_ability_system(
    mut commands: Commands,
    mut cancel_cast_ability_event_reader: EventReader<CancelCastAbility>,
) {
    for cancel_cast_ability in cancel_cast_ability_event_reader.iter() {
        commands
            .entity(cancel_cast_ability.source)
            .remove::<CastAbility>();
    }
}

fn perform_ability_system(
    mut commands: Commands,
    mut perform_ability_event_reader: EventReader<PerformAbility>,
    mut perform_effect_event_writer: EventWriter<PerformEffect>,
    mut query: Query<(&Transform, &mut Mana, &mut AbilityCooldowns)>,
    creature_query: Query<(Entity, &Transform), With<Creature>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
) {
    for perform_ability in perform_ability_event_reader.iter() {
        // TODO: Verify target position in case it moves while casting.

        let (transform, mut mana, mut ability_cooldowns) =
            query.get_mut(perform_ability.source).unwrap();

        mana.points -= perform_ability.ability.mana_points;

        commands
            .entity(perform_ability.source)
            .insert(RegenManaCooldown::new());

        if perform_ability.ability.cooldown_duration > 0.0 {
            ability_cooldowns.push(perform_ability.ability);
        }

        let mut effects = vec![perform_ability.ability.effect];
        if let Some(secondary_effect) = perform_ability.ability.secondary_effect {
            effects.push(secondary_effect);
        }

        for (effect, effect_targeting) in effects.iter() {
            let targets = match effect_targeting {
                AbilityTargetMode::Single => vec![perform_ability.target.unwrap()],
                AbilityTargetMode::Area => {
                    let position = transform.translation.truncate();

                    creature_query
                        .iter()
                        .filter(|(creature_entity, creature_transform)| {
                            *creature_entity != perform_ability.source
                                && verify_target_position(
                                    position,
                                    creature_transform.translation.truncate(),
                                    perform_ability.ability.range,
                                    &obstacle_query,
                                )
                                .is_ok()
                        })
                        .map(|(creature_entity, _)| creature_entity)
                        .collect()
                }
            };

            for target in &targets {
                perform_effect_event_writer.send(PerformEffect {
                    source: perform_ability.source,
                    effect: *effect,
                    target: *target,
                });
            }
        }

        let ability_name = perform_ability.ability.name;
        info!("Casted {ability_name}.");
    }
}

enum TargetPositionError {
    Range,
    Sight,
}

fn verify_target_position(
    position: Vec2,
    target_position: Vec2,
    range: f32,
    obstacle_query: &Query<&Transform, With<Obstacle>>,
) -> Result<(), TargetPositionError> {
    let direction = target_position - position;
    let direction_length = direction.length();
    if direction_length > range {
        return Err(TargetPositionError::Range);
    }

    if obstacle_query.iter().any(|obstacle_transform| {
        is_intersecting(
            position,
            target_position,
            obstacle_transform.translation.truncate(),
            Vec2::splat(crate::zone::Tile::SIZE),
        )
    }) {
        return Err(TargetPositionError::Sight);
    }

    Ok(())
}
