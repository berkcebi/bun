use crate::creature::CREATURE_SPEED;
use bevy::prelude::*;

/// Event to change position towards a direction.
pub struct ChangePosition {
    pub entity: Entity,
    pub direction: Vec3,
}

/// Component to indicate position changing.
#[derive(Component)]
pub struct ChangingPosition;

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangePosition>()
            .add_system(change_position);
    }
}

fn change_position(
    mut commands: Commands,
    time: Res<Time>,
    mut change_position_event_reader: EventReader<ChangePosition>,
    mut query: Query<(Entity, &mut Transform, Option<&ChangingPosition>)>,
    mut changing_position_query: Query<Entity, With<ChangingPosition>>,
) {
    let mut entities_changing_position = vec![];
    for change_position in change_position_event_reader.iter() {
        let (entity, mut transform, moving) = query.get_mut(change_position.entity).unwrap();
        transform.translation += change_position.direction * time.delta_seconds() * CREATURE_SPEED;

        if moving.is_none() {
            commands.entity(entity).insert(ChangingPosition);
        }

        entities_changing_position.push(entity);
    }

    for entity in changing_position_query.iter_mut() {
        if !entities_changing_position.contains(&entity) {
            commands.entity(entity).remove::<ChangingPosition>();
        }
    }
}
