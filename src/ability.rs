use crate::{
    effect::{AffectTarget, Effect},
    mana::{Mana, RegenManaCooldown},
};
use bevy::prelude::*;

const USE_ABILITY_COOLDOWN_DURATION: f32 = 1.5;

#[derive(Clone, Copy)]
pub struct Ability {
    pub name: &'static str,
    pub mana_points: u8,
    pub use_duration: f32,
    pub effect: Effect,
    pub secondary_effect: Option<Effect>,
}

pub struct TryAbility {
    pub source: Entity,
    pub ability: Ability,
    pub target: Entity,
}

struct UseAbility {
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
            duration_timer: Timer::from_seconds(ability.use_duration, false),
        }
    }
}

struct UseAbilityCooldown {
    duration_timer: Timer,
}

impl Default for UseAbilityCooldown {
    fn default() -> Self {
        Self {
            duration_timer: Timer::from_seconds(USE_ABILITY_COOLDOWN_DURATION, false),
        }
    }
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<TryAbility>()
            .add_event::<UseAbility>()
            .add_system(remove_use_ability_cooldown.system())
            .add_system(try_ability.system())
            .add_system(cast_ability.system())
            .add_system(use_ability.system());
    }
}

fn remove_use_ability_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut UseAbilityCooldown)>,
) {
    for (entity, mut use_ability_cooldown) in query.iter_mut() {
        use_ability_cooldown.duration_timer.tick(time.delta());

        if use_ability_cooldown.duration_timer.finished() {
            info!("Global cooldown over.");
            commands.entity(entity).remove::<UseAbilityCooldown>();
        }
    }
}

fn try_ability(
    mut commands: Commands,
    mut try_ability_event_reader: EventReader<TryAbility>,
    mut use_ability_event_writer: EventWriter<UseAbility>,
    mut query: Query<(&Mana, Option<&CastAbility>, Option<&UseAbilityCooldown>)>,
) {
    for try_ability in try_ability_event_reader.iter() {
        let (mana, cast_ability, use_ability_cooldown) = query.get_mut(try_ability.source).unwrap();

        if cast_ability.is_some() {
            info!("Casting another ability.");

            continue;
        }

        if use_ability_cooldown.is_some() {
            info!("Under global cooldown.");

            continue;
        }

        if try_ability.ability.mana_points > mana.points {
            info!("Not enough mana.");

            continue;
        }

        commands
            .entity(try_ability.source)
            .insert(UseAbilityCooldown::default());

        if try_ability.ability.use_duration > 0.0 {
            commands
                .entity(try_ability.source)
                .insert(CastAbility::new(try_ability.ability, try_ability.target));
        } else {
            use_ability_event_writer.send(UseAbility {
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
    mut use_ability_event_writer: EventWriter<UseAbility>,
    mut query: Query<(Entity, &mut CastAbility)>,
) {
    for (entity, mut cast_ability) in query.iter_mut() {
        cast_ability.duration_timer.tick(time.delta());

        if cast_ability.duration_timer.finished() {
            commands.entity(entity).remove::<CastAbility>();

            use_ability_event_writer.send(UseAbility {
                source: entity,
                ability: cast_ability.ability,
                target: cast_ability.target,
            });
        }
    }
}

fn use_ability(
    mut commands: Commands,
    mut use_ability_event_reader: EventReader<UseAbility>,
    mut affect_target_event_writer: EventWriter<AffectTarget>,
    mut query: Query<&mut Mana>,
) {
    for use_ability in use_ability_event_reader.iter() {
        let mut mana = query.get_mut(use_ability.source).unwrap();

        mana.points -= use_ability.ability.mana_points;

        commands
            .entity(use_ability.source)
            .insert(RegenManaCooldown::new());

        affect_target_event_writer.send(AffectTarget {
            source: use_ability.source,
            effect: use_ability.ability.effect,
            target: use_ability.target,
        });

        if let Some(secondary_effect) = use_ability.ability.secondary_effect {
            affect_target_event_writer.send(AffectTarget {
                source: use_ability.source,
                effect: secondary_effect,
                target: use_ability.target,
            });
        }

        info!("Casted {}.", use_ability.ability.name);
    }
}
