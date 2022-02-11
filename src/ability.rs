use crate::{
    effect::{Effect, LastingEffect, LastingEffects, PerformEffect},
    mana::{Mana, RegenManaCooldown},
    position::ChangingPosition,
    AppState,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const ABILITY_COOLDOWN_DURATION: f32 = 1.5;

#[derive(Clone, Copy)]
pub struct Ability {
    pub name: &'static str,
    pub mana_points: u16,
    pub cast_duration: f32,
    pub range: f32,
    pub effect: Effect,
    pub secondary_effect: Option<Effect>,
}

/// Event to initiate an ability, if possible.
pub struct TryAbility {
    pub source: Entity,
    pub ability: Ability,
    pub target: Option<Entity>,
}

/// Internal event to perform an ability via a try ability event.
struct PerformAbility {
    source: Entity,
    ability: Ability,
    target: Entity,
}

/// Component to store cast duration for an ability.
#[derive(Component)]
pub struct CastAbility {
    pub ability: Ability,
    pub target: Entity,
    pub duration_timer: Timer,
}

impl CastAbility {
    pub fn new(ability: Ability, target: Entity) -> Self {
        Self {
            ability,
            target,
            duration_timer: Timer::from_seconds(ability.cast_duration, false),
        }
    }
}

/// Component to disable casting for a defined duration, i.e. global cooldown.
#[derive(Component)]
struct AbilityCooldown {
    duration_timer: Timer,
}

impl Default for AbilityCooldown {
    fn default() -> Self {
        Self {
            duration_timer: Timer::from_seconds(ABILITY_COOLDOWN_DURATION, false),
        }
    }
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TryAbility>()
            .add_event::<PerformAbility>()
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(remove_ability_cooldown_system)
                    .with_system(try_ability_system)
                    .with_system(cast_ability_system)
                    .with_system(perform_ability_system),
            );
    }
}

fn remove_ability_cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AbilityCooldown)>,
) {
    for (entity, mut ability_cooldown) in query.iter_mut() {
        ability_cooldown.duration_timer.tick(time.delta());

        if ability_cooldown.duration_timer.finished() {
            info!("Global cooldown over.");
            commands.entity(entity).remove::<AbilityCooldown>();
        }
    }
}

fn try_ability_system(
    mut commands: Commands,
    query_pipeline: Res<QueryPipeline>,
    mut try_ability_event_reader: EventReader<TryAbility>,
    mut perform_ability_event_writer: EventWriter<PerformAbility>,
    mut query: Query<(
        &Mana,
        &LastingEffects,
        &Transform,
        Option<&CastAbility>,
        Option<&AbilityCooldown>,
        Option<&ChangingPosition>,
    )>,
    target_query: Query<&Transform>,
    collider_query: QueryPipelineColliderComponentsQuery,
) {
    for try_ability in try_ability_event_reader.iter() {
        let target = match try_ability.target {
            Some(result) => result,
            None => {
                info!("No target.");

                continue;
            }
        };

        let (mana, lasting_effects, transform, cast_ability, ability_cooldown, changing_position) =
            query.get_mut(try_ability.source).unwrap();

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

        if ability_cooldown.is_some() {
            info!("Under global cooldown.");

            continue;
        }

        if try_ability.ability.mana_points > mana.points {
            info!("Not enough mana.");

            continue;
        }

        if target != try_ability.source {
            let position = transform.translation.truncate();

            let target_transform = target_query.get(target).unwrap();
            let target_position = target_transform.translation.truncate();

            let direction = target_position - position;
            let direction_length = direction.length();
            if direction_length > try_ability.ability.range {
                info!("Out of range.");

                continue;
            }

            let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
            let ray = Ray::new(position.into(), direction.normalize().into());
            if query_pipeline
                .cast_ray(
                    &collider_set,
                    &ray,
                    direction_length,
                    true,
                    InteractionGroups::all(),
                    None,
                )
                .is_some()
            {
                info!("Not in line of sight.");

                continue;
            }
        }

        commands
            .entity(try_ability.source)
            .insert(AbilityCooldown::default());

        if try_ability.ability.cast_duration > 0.0 {
            commands
                .entity(try_ability.source)
                .insert(CastAbility::new(try_ability.ability, target));
        } else {
            perform_ability_event_writer.send(PerformAbility {
                source: try_ability.source,
                ability: try_ability.ability,
                target,
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

fn perform_ability_system(
    mut commands: Commands,
    mut perform_ability_event_reader: EventReader<PerformAbility>,
    mut perform_effect_event_writer: EventWriter<PerformEffect>,
    mut query: Query<&mut Mana>,
) {
    for perform_ability in perform_ability_event_reader.iter() {
        // TODO: Validate range in case the target moves while casting.

        let mut mana = query.get_mut(perform_ability.source).unwrap();

        mana.points -= perform_ability.ability.mana_points;

        commands
            .entity(perform_ability.source)
            .insert(RegenManaCooldown::new());

        let mut effects = vec![perform_ability.ability.effect];
        if let Some(secondary_effect) = perform_ability.ability.secondary_effect {
            effects.push(secondary_effect);
        }

        for effect in effects.iter() {
            perform_effect_event_writer.send(PerformEffect {
                source: perform_ability.source,
                effect: *effect,
                target: perform_ability.target,
            });
        }

        let ability_name = perform_ability.ability.name;
        info!("Casted {ability_name}.");
    }
}
