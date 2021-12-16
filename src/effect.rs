use crate::health::Health;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Effect {
    LoseHealth { points: u8 },
    GainHealth { points: u8 },
}

pub struct AffectTarget {
    pub target: Entity,
    pub effect: Effect,
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
    mut health_query: Query<&mut Health>,
) {
    for affect_target in affect_target_event_reader.iter() {
        let target = affect_target.target;

        match affect_target.effect {
            Effect::LoseHealth { points } => {
                let mut health = health_query.get_mut(target).unwrap();

                if health.points > points {
                    health.points -= points;
                } else {
                    // TODO: Remove from game.
                    health.points = 0;

                    info!("{:?} died.", target);
                }
            }
            Effect::GainHealth { points } => {
                let mut health = health_query.get_mut(target).unwrap();

                health.points = (health.points + points).min(health.max_points);
            }
        }
    }
}
