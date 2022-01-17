use crate::{
    critical::Critical,
    effect::{LastingEffects, PeriodicMomentaryEffects},
    health::Health,
    mana::Mana,
    target::Target,
};
use bevy::prelude::*;

pub const CREATURE_SPEED: f32 = 60.0;

#[derive(Bundle)]
pub struct CreatureBundle {
    health: Health,
    mana: Mana,
    critical: Critical,
    periodic_momentary_effects: PeriodicMomentaryEffects,
    lasting_effects: LastingEffects,
    target: Target,
}

impl CreatureBundle {
    pub fn new(health_points: u16, mana_points: u16) -> Self {
        Self {
            health: Health::new(health_points),
            mana: Mana::new(mana_points),
            critical: Critical::default(),
            periodic_momentary_effects: PeriodicMomentaryEffects::default(),
            lasting_effects: LastingEffects::default(),
            target: Target::default(),
        }
    }
}
