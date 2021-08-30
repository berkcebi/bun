use crate::ability::UseAbility;
use bevy::prelude::*;

const REGEN_MANA_INTERVAL: f64 = 0.5;
const REGEN_MANA_COOLDOWN_DURATION: f32 = 5.0;

pub struct Mana {
    pub points: u8,
    pub max_points: u8,
    pub regen_points: u8,
}

pub struct RegenManaCooldown {
    duration_timer: Timer,
}

impl RegenManaCooldown {
    pub fn new() -> Self {
        Self {
            duration_timer: Timer::from_seconds(REGEN_MANA_COOLDOWN_DURATION, false),
        }
    }
}

pub struct ManaPlugin;

impl Plugin for ManaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(bevy::core::FixedTimestep::step(REGEN_MANA_INTERVAL))
                .with_system(regen_mana.system()),
        )
        .add_system(remove_regen_mana_cooldown.system());
    }
}

fn regen_mana(mut query: Query<&mut Mana, (Without<UseAbility>, Without<RegenManaCooldown>)>) {
    for mut mana in query.iter_mut() {
        if mana.points < mana.max_points {
            mana.points = (mana.points + mana.regen_points).min(mana.max_points);
        }
    }
}

fn remove_regen_mana_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut RegenManaCooldown)>,
) {
    for (entity, mut regen_mana_cooldown) in query.iter_mut() {
        regen_mana_cooldown.duration_timer.tick(time.delta());

        if regen_mana_cooldown.duration_timer.finished() {
            commands.entity(entity).remove::<RegenManaCooldown>();
        }
    }
}
