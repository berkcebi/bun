use crate::{
    ability::AbilityCooldowns,
    critical::Critical,
    effect::{LastingEffects, PeriodicMomentaryEffects},
    health::Health,
    mana::Mana,
    target::Target,
};
use bevy::prelude::*;

pub const CREATURE_SPEED: f32 = 60.0;

#[derive(Component)]
pub struct Creature;

#[derive(Bundle)]
pub struct CreatureBundle {
    creature: Creature,
    health: Health,
    mana: Mana,
    critical: Critical,
    periodic_momentary_effects: PeriodicMomentaryEffects,
    lasting_effects: LastingEffects,
    ability_cooldowns: AbilityCooldowns,
    target: Target,
}

impl CreatureBundle {
    pub fn new(health_points: u16, mana_points: u16) -> Self {
        Self {
            creature: Creature,
            health: Health::new(health_points),
            mana: Mana::new(mana_points),
            critical: Critical::default(),
            periodic_momentary_effects: PeriodicMomentaryEffects::default(),
            lasting_effects: LastingEffects::default(),
            ability_cooldowns: AbilityCooldowns::default(),
            target: Target::default(),
        }
    }
}
