use crate::{
    action::{Action, TargetAction},
    mana::{Mana, RegenManaCooldown},
};
use bevy::prelude::*;

const USE_ABILITY_COOLDOWN_DURATION: f32 = 1.5;

#[derive(Clone, Copy)]
pub struct Ability {
    pub name: &'static str,
    pub mana_points: u8,
    pub use_duration: f32,
    // FIXME: Change to array.
    pub action: Action,
}

impl Ability {
    pub const FIREBALL: Self = Self {
        name: "Fireball",
        mana_points: 25,
        use_duration: 2.5,
        action: Action::LoseHealth { points: 10 },
    };

    pub const FIRE_BLAST: Self = Self {
        name: "Fire Blast",
        mana_points: 10,
        use_duration: 0.0,
        action: Action::LoseHealth { points: 5 },
    };

    pub const LESSER_HEAL: Self = Self {
        name: "Lesser Heal",
        mana_points: 15,
        use_duration: 1.5,
        action: Action::GainHealth { points: 20 },
    };
}

pub struct UseAbility {
    pub ability: Ability,
    pub target: Entity,
    pub duration_timer: Timer,
}

impl UseAbility {
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
        app.add_system(use_ability.system())
            .add_system(remove_use_ability_cooldown.system());
    }
}

fn use_ability(
    mut commands: Commands,
    time: Res<Time>,
    mut target_action_event_writer: EventWriter<TargetAction>,
    mut query: Query<(
        Entity,
        &mut UseAbility,
        &mut Mana,
        Option<&UseAbilityCooldown>,
    )>,
) {
    for (entity, mut use_ability, mut mana, use_ability_cooldown) in query.iter_mut() {
        let ability = use_ability.ability;

        if use_ability.duration_timer.elapsed_secs() <= 0.0 {
            if use_ability_cooldown.is_some() {
                info!("Under global cooldown.");

                commands.entity(entity).remove::<UseAbility>();
                continue;
            }

            if ability.mana_points > mana.points {
                info!("Not enough mana.");

                commands.entity(entity).remove::<UseAbility>();
                continue;
            }

            commands
                .entity(entity)
                .insert(UseAbilityCooldown::default());
        }

        use_ability.duration_timer.tick(time.delta());

        if use_ability.duration_timer.finished() {
            mana.points -= ability.mana_points;

            info!("Casted {}.", ability.name);

            commands.entity(entity).remove::<UseAbility>();
            commands.entity(entity).insert(RegenManaCooldown::new());

            target_action_event_writer.send(TargetAction {
                target: use_ability.target,
                action: use_ability.ability.action,
            });
        }
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
