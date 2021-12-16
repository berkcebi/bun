use crate::health::Health;
use bevy::prelude::*;

pub struct TargetAction {
    pub target: Entity,
    pub action: Action,
}

#[derive(Clone, Copy)]
pub enum Action {
    LoseHealth { points: u8 },
    GainHealth { points: u8 },
}

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<TargetAction>()
            .add_system(perform_target_action.system());
    }
}

fn perform_target_action(
    mut target_action_event_reader: EventReader<TargetAction>,
    mut health_query: Query<&mut Health>,
) {
    for target_action in target_action_event_reader.iter() {
        let target = target_action.target;

        match target_action.action {
            Action::LoseHealth { points } => {
                let mut health = health_query.get_mut(target).unwrap();

                if health.points > points {
                    health.points -= points;
                } else {
                    // TODO: Remove from game.
                    health.points = 0;

                    info!("{:?} died.", target);
                }
            }
            Action::GainHealth { points } => {
                let mut health = health_query.get_mut(target).unwrap();

                health.points = (health.points + points).min(health.max_points);
            }
        }
    }
}
