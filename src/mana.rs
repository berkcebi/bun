use crate::{ability::CastAbility, AppState};
use bevy::prelude::*;

const REGEN_MANA_POINTS: u16 = 1;
const REGEN_MANA_INTERVAL: f64 = 0.5;
const REGEN_MANA_COOLDOWN_DURATION: f32 = 5.0;

#[derive(Component)]
pub struct Mana {
    pub points: u16,
    pub max_points: u16,
    pub regen_points: u16,
}

impl Mana {
    pub fn new(points: u16) -> Self {
        Self {
            points,
            max_points: points,
            regen_points: REGEN_MANA_POINTS,
        }
    }
}

/// Component to disable mana regeneration for a defined duration.
#[derive(Component)]
pub struct RegenManaCooldown {
    duration_timer: Timer,
}

impl RegenManaCooldown {
    pub fn new() -> Self {
        Self {
            duration_timer: Timer::from_seconds(REGEN_MANA_COOLDOWN_DURATION, TimerMode::Once),
        }
    }
}

pub struct ManaPlugin;

impl Plugin for ManaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_run_criteria(bevy::time::FixedTimestep::step(REGEN_MANA_INTERVAL))
                .with_system(regen_mana_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game).with_system(remove_regen_mana_cooldown_system),
        );
    }
}

fn regen_mana_system(
    mut query: Query<&mut Mana, (Without<CastAbility>, Without<RegenManaCooldown>)>,
) {
    for mut mana in query.iter_mut() {
        if mana.points < mana.max_points {
            mana.points = (mana.points + mana.regen_points).min(mana.max_points);
        }
    }
}

fn remove_regen_mana_cooldown_system(
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
