use crate::{creature::CREATURE_SPEED, sprite::Sprite, AppState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    query_pipeline: Res<QueryPipeline>,
    mut change_position_event_reader: EventReader<ChangePosition>,
    mut query: Query<(Entity, &mut Transform, Option<&ChangingPosition>)>,
    mut changing_position_query: Query<Entity, With<ChangingPosition>>,
    collider_query: QueryPipelineColliderComponentsQuery,
) {
    let mut entities_changing_position = vec![];
    for change_position in change_position_event_reader.iter() {
        let (entity, mut transform, moving) = query.get_mut(change_position.entity).unwrap();

        let mut changed_position = false;
        for unit_direction in [Vec3::X, Vec3::Y] {
            let translation_delta = change_position.direction.extend(0.0)
                * unit_direction
                * time.delta_seconds()
                * CREATURE_SPEED;

            if !intersects_with_collider(
                transform.translation + translation_delta,
                &query_pipeline,
                &collider_query,
            ) {
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

fn intersects_with_collider(
    translation: Vec3,
    query_pipeline: &Res<QueryPipeline>,
    query: &QueryPipelineColliderComponentsQuery,
) -> bool {
    let collider_set = QueryPipelineColliderComponentsSet(query);
    let cuboid_half_extent = Sprite::SIZE / 2.0;
    let cuboid = Cuboid::new(Vec2::new(cuboid_half_extent, cuboid_half_extent).into());
    let cuboid_position = translation.truncate().into();

    let mut intersects = false;
    query_pipeline.intersections_with_shape(
        &collider_set,
        &cuboid_position,
        &cuboid,
        InteractionGroups::all(),
        None,
        |_| {
            intersects = true;

            false
        },
    );

    intersects
}
