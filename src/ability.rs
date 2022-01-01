use crate::{
    effect::{Effect, LastingEffect, LastingEffects, PerformEffect},
    mana::{Mana, RegenManaCooldown},
};
use bevy::prelude::*;

const ABILITY_COOLDOWN_DURATION: f32 = 1.5;

#[derive(Clone, Copy)]
pub struct Ability {
    pub name: &'static str,
    pub mana_points: u8,
    pub cast_duration: f32,
    pub effect: Effect,
    pub secondary_effect: Option<Effect>,
}

pub struct TryAbility {
    pub source: Entity,
    pub ability: Ability,
    pub target: Entity,
}

struct PerformAbility {
    source: Entity,
    ability: Ability,
    target: Entity,
}

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
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<TryAbility>()
            .add_event::<PerformAbility>()
            .add_system(remove_ability_cooldown.system())
            .add_system(try_ability.system())
            .add_system(cast_ability.system())
            .add_system(perform_ability.system());
    }
}

fn remove_ability_cooldown(
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

fn try_ability(
    mut commands: Commands,
    mut try_ability_event_reader: EventReader<TryAbility>,
    mut perform_ability_event_writer: EventWriter<PerformAbility>,
    mut query: Query<(
        &Mana,
        &LastingEffects,
        Option<&CastAbility>,
        Option<&AbilityCooldown>,
    )>,
) {
    for try_ability in try_ability_event_reader.iter() {
        let (mana, lasting_effects, cast_ability, ability_cooldown) =
            query.get_mut(try_ability.source).unwrap();

        if cast_ability.is_some() {
            info!("Casting another ability.");

            continue;
        }

        if lasting_effects
            .instances
            .iter()
            .any(|instance| instance.lasting_effect == LastingEffect::Silence)
        {
            info!("Silenced.");

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

        commands
            .entity(try_ability.source)
            .insert(AbilityCooldown::default());

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

fn cast_ability(
    mut commands: Commands,
    time: Res<Time>,
    mut perform_ability_event_writer: EventWriter<PerformAbility>,
    mut query: Query<(Entity, &mut CastAbility)>,
) {
    for (entity, mut cast_ability) in query.iter_mut() {
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

fn perform_ability(
    mut commands: Commands,
    mut perform_ability_event_reader: EventReader<PerformAbility>,
    mut perform_effect_event_writer: EventWriter<PerformEffect>,
    mut query: Query<&mut Mana>,
) {
    for perform_ability in perform_ability_event_reader.iter() {
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

        info!("Casted {}.", perform_ability.ability.name);
    }
}
