use crate::{
    critical::{Critical, CRITICAL_MULTIPLIER},
    health::Health,
};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub enum Effect {
    LoseHealth { min_points: u8, max_points: u8 },
    GainHealth { min_points: u8, max_points: u8 },
}

pub struct AffectTarget {
    pub source: Entity,
    pub effect: Effect,
    pub target: Entity,
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AffectTarget>()
            .add_system(affect_target.system());
    }
}

fn affect_target(
    mut affect_target_event_reader: EventReader<AffectTarget>,
    mut critical_query: Query<Option<&Critical>>,
    mut health_query: Query<&mut Health>,
) {
    let mut rng = rand::thread_rng();

    for affect_target in affect_target_event_reader.iter() {
        let target = affect_target.target;

        match affect_target.effect {
            Effect::LoseHealth {
                min_points,
                max_points,
            } => {
                let mut health = health_query.get_mut(target).unwrap();
                let mut points = rng.gen_range(min_points..=max_points);

                let critical = critical_query.get_mut(affect_target.source).unwrap();
                if let Some(critical) = critical {
                    if critical.percentage >= rng.gen() {
                        points *= CRITICAL_MULTIPLIER;
                    }
                }

                if health.points > points {
                    health.points -= points;
                } else {
                    // TODO: Remove from game.
                    health.points = 0;

                    info!("{:?} died.", target);
                }
            }
            Effect::GainHealth {
                min_points,
                max_points,
            } => {
                let mut health = health_query.get_mut(target).unwrap();
                let mut points = rng.gen_range(min_points..=max_points);

                let critical = critical_query.get_mut(affect_target.source).unwrap();
                if let Some(critical) = critical {
                    if critical.percentage >= rng.gen() {
                        points *= CRITICAL_MULTIPLIER;
                    }
                }

                health.points = (health.points + points).min(health.max_points);
            }
        }
    }
}
