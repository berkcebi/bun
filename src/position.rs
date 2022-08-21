use crate::{
    creature::{Creature, CREATURE_SPEED},
    level::Obstacle,
    AppState,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};

/// Event to change position towards a direction.
pub struct ChangePosition {
    pub entity: Entity,
    pub direction: Vec2,
}

/// Component to indicate position changing.
#[derive(Component)]
pub struct ChangingPosition;

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangePosition>().add_system_set(
            SystemSet::on_update(AppState::Game).with_system(change_position_system),
        );
    }
}

fn change_position_system(
    mut commands: Commands,
    time: Res<Time>,
    mut change_position_event_reader: EventReader<ChangePosition>,
    mut query: Query<(Entity, &mut Transform, Option<&ChangingPosition>), With<Creature>>,
    mut changing_position_query: Query<Entity, (With<ChangingPosition>, With<Creature>)>,
    obstacle_query: Query<&Transform, (With<Obstacle>, Without<Creature>)>,
) {
    let mut entities_changing_position = vec![];
    for change_position in change_position_event_reader.iter() {
        let (entity, mut transform, moving) = query.get_mut(change_position.entity).unwrap();

        let is_colliding = |translation| {
            obstacle_query.iter().any(|obstacle_transform| {
                collide(
                    translation,
                    Vec2::splat(crate::zone::Tile::SIZE),
                    obstacle_transform.translation,
                    Vec2::splat(crate::zone::Tile::SIZE),
                )
                .is_some()
            })
        };

        let mut changed_position = false;
        for unit_direction in [Vec3::X, Vec3::Y] {
            let translation_delta = change_position.direction.extend(0.0)
                * unit_direction
                * time.delta_seconds()
                * CREATURE_SPEED;

            if !is_colliding(transform.translation + translation_delta) {
                transform.translation += translation_delta;
                changed_position = true;
            }
        }

        if !changed_position {
            continue;
        }

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
