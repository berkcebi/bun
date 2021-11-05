use bevy::prelude::*;

use crate::health::Health;

// FIXME: Consider using an enum instead, shared with Ability.
pub struct LoseHealth {
    pub target: Entity,
    pub points: u8,
}

pub struct GainHealth {
    pub target: Entity,
    pub points: u8,
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<LoseHealth>()
            .add_event::<GainHealth>()
            .add_system(lose_health.system())
            .add_system(gain_health.system());
    }
}

fn lose_health(
    mut lose_health_event_reader: EventReader<LoseHealth>,
    mut query: Query<&mut Health>,
) {
    for lose_health in lose_health_event_reader.iter() {
        let mut health = match query.get_mut(lose_health.target) {
            Ok(result) => result,
            Err(_) => {
                error!("Querying health component for lose health event target failed.");

                continue;
            }
        };

        if health.points > lose_health.points {
            health.points = health.points - lose_health.points;
        } else {
            // TODO: Remove from game.
            health.points = 0;

            info!("{:?} died.", lose_health.target);
        }
    }
}

fn gain_health(
    mut gain_health_event_reader: EventReader<GainHealth>,
    mut query: Query<&mut Health>,
) {
    for gain_health in gain_health_event_reader.iter() {
        let mut health = match query.get_mut(gain_health.target) {
            Ok(result) => result,
            Err(_) => {
                error!("Querying health component for gain health event target failed.");

                continue;
            }
        };

        health.points = (health.points + gain_health.points).min(health.max_points);
    }
}
