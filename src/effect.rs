use crate::{
    critical::{Critical, CRITICAL_MULTIPLIER},
    health::Health,
};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub enum Effect {
    Momentary(MomentaryEffect, MomentaryEffectSchedule),
    Lasting(LastingEffect, f32),
}

#[derive(Clone, Copy)]
pub enum MomentaryEffect {
    Damage(u16, u16),
    Heal(u16, u16),
}

#[derive(Clone, Copy)]
pub enum MomentaryEffectSchedule {
    Once,
    Periodic(f32, f32),
}

#[derive(Clone, Copy)]
pub enum LastingEffect {
    Silence,
}

/// Event to perform an effect, usually when an ability is cast.
pub struct PerformEffect {
    pub effect: Effect,
    pub source: Entity,
    pub target: Entity,
}

pub enum PerformedMomentaryEffect {
    Damage(u16, bool),
    Heal(u16, bool),
}

/// Event to communicate performing momentary effect on an entity.
pub struct MomentaryEffectPerformed {
    pub entity: Entity,
    pub performed_momentary_effect: PerformedMomentaryEffect,
}

/// Internal event to perform a momentary effect via a perform effect event.
struct PerformMomentaryEffect {
    pub effect: MomentaryEffect,
    pub source: Entity,
    pub target: Entity,
}

/// Component to store ongoing periodic momentary effects.
#[derive(Component, Default)]
pub struct PeriodicMomentaryEffects {
    instances: Vec<PeriodicMomentaryEffectInstance>,
}

struct PeriodicMomentaryEffectInstance {
    effect: MomentaryEffect,
    interval_timer: Timer,
    duration_timer: Timer,
    source: Entity,
}

/// Component to store ongoing lasting effects.
#[derive(Component, Default)]
pub struct LastingEffects {
    pub instances: Vec<LastingEffectInstance>,
}

pub struct LastingEffectInstance {
    pub effect: LastingEffect,
    pub duration_timer: Timer,
    pub source: Entity,
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PerformEffect>()
            .add_event::<MomentaryEffectPerformed>()
            .add_event::<PerformMomentaryEffect>()
            .add_system(perform_effect_system)
            .add_system(perform_momentary_effect_system)
            .add_system(tick_periodic_momentary_effects_system)
            .add_system(tick_lasting_effects_system);
    }
}

fn perform_effect_system(
    mut perform_effect_event_reader: EventReader<PerformEffect>,
    mut perform_momentary_effect_event_writer: EventWriter<PerformMomentaryEffect>,
    mut periodic_momentary_effects_query: Query<&mut PeriodicMomentaryEffects>,
    mut lasting_effects_query: Query<&mut LastingEffects>,
) {
    for perform_effect in perform_effect_event_reader.iter() {
        match perform_effect.effect {
            Effect::Momentary(effect, MomentaryEffectSchedule::Once) => {
                perform_momentary_effect_event_writer.send(PerformMomentaryEffect {
                    effect,
                    source: perform_effect.source,
                    target: perform_effect.target,
                })
            }
            Effect::Momentary(effect, MomentaryEffectSchedule::Periodic(interval, duration)) => {
                let mut periodic_momentary_effects = periodic_momentary_effects_query
                    .get_mut(perform_effect.target)
                    .unwrap();
                periodic_momentary_effects
                    .instances
                    .push(PeriodicMomentaryEffectInstance {
                        effect,
                        interval_timer: Timer::from_seconds(interval, true),
                        duration_timer: Timer::from_seconds(duration, false),
                        source: perform_effect.source,
                    })
            }
            Effect::Lasting(effect, duration) => {
                let mut lasting_effects = lasting_effects_query
                    .get_mut(perform_effect.target)
                    .unwrap();
                lasting_effects.instances.push(LastingEffectInstance {
                    effect,
                    duration_timer: Timer::from_seconds(duration, false),
                    source: perform_effect.source,
                })
            }
        }
    }
}

fn perform_momentary_effect_system(
    mut perform_momentary_effect_event_reader: EventReader<PerformMomentaryEffect>,
    mut momentary_effect_performed_event_writer: EventWriter<MomentaryEffectPerformed>,
    mut critical_query: Query<Option<&Critical>>,
    mut health_query: Query<&mut Health>,
) {
    let mut rng = rand::thread_rng();

    for perform_momentary_effect in perform_momentary_effect_event_reader.iter() {
        let target = perform_momentary_effect.target;

        match perform_momentary_effect.effect {
            MomentaryEffect::Damage(min_points, max_points) => {
                let mut health = health_query.get_mut(target).unwrap();
                let mut points = rng.gen_range(min_points..=max_points);

                let critical = critical_query
                    .get_mut(perform_momentary_effect.source)
                    .unwrap();
                let is_critical = match critical {
                    Some(critical) => critical.percent >= rng.gen(),
                    None => false,
                };

                if is_critical {
                    points *= CRITICAL_MULTIPLIER;
                }

                if health.points > points {
                    health.points -= points;
                } else {
                    // TODO: Remove from game.
                    health.points = 0;

                    info!("{:?} died.", target);
                }

                momentary_effect_performed_event_writer.send(MomentaryEffectPerformed {
                    entity: target,
                    performed_momentary_effect: PerformedMomentaryEffect::Damage(
                        points,
                        is_critical,
                    ),
                });
            }
            MomentaryEffect::Heal(min_points, max_points) => {
                let mut health = health_query.get_mut(target).unwrap();
                let mut points = rng.gen_range(min_points..=max_points);

                let critical = critical_query
                    .get_mut(perform_momentary_effect.source)
                    .unwrap();
                let is_critical = match critical {
                    Some(critical) => critical.percent >= rng.gen(),
                    None => false,
                };

                if is_critical {
                    points *= CRITICAL_MULTIPLIER;
                }

                health.points = (health.points + points).min(health.max_points);

                momentary_effect_performed_event_writer.send(MomentaryEffectPerformed {
                    entity: target,
                    performed_momentary_effect: PerformedMomentaryEffect::Heal(points, is_critical),
                });
            }
        }
    }
}

fn tick_periodic_momentary_effects_system(
    time: Res<Time>,
    mut perform_momentary_effect_event_writer: EventWriter<PerformMomentaryEffect>,
    mut query: Query<(Entity, &mut PeriodicMomentaryEffects)>,
) {
    for (entity, mut periodic_momentary_effects) in query.iter_mut() {
        let instances = &mut periodic_momentary_effects.instances;
        for instance in instances.iter_mut() {
            instance.interval_timer.tick(time.delta());
            if instance.interval_timer.finished() {
                perform_momentary_effect_event_writer.send(PerformMomentaryEffect {
                    effect: instance.effect,
                    source: instance.source,
                    target: entity,
                });
            }

            instance.duration_timer.tick(time.delta());
        }

        instances.retain(|instance| !instance.duration_timer.finished());
    }
}

fn tick_lasting_effects_system(time: Res<Time>, mut query: Query<&mut LastingEffects>) {
    for mut lasting_effects in query.iter_mut() {
        let instances = &mut lasting_effects.instances;
        for instance in instances.iter_mut() {
            instance.duration_timer.tick(time.delta());
        }

        instances.retain(|instance| !instance.duration_timer.finished());
    }
}
