use crate::{
    critical::{Critical, CRITICAL_MULTIPLIER},
    health::Health,
};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Clone, Copy)]
pub enum Effect {
    Momentary {
        momentary_effect: MomentaryEffect,
    },
    PeriodicMomentary {
        momentary_effect: MomentaryEffect,
        interval: f32,
        duration: f32,
    },
    Lasting {
        lasting_effect: LastingEffect,
        duration: f32,
    },
}

#[derive(Clone, Copy)]
pub enum MomentaryEffect {
    LoseHealth { min_points: u8, max_points: u8 },
    GainHealth { min_points: u8, max_points: u8 },
}

#[derive(Clone, Copy, PartialEq)]
pub enum LastingEffect {
    Silence,
}

pub struct PerformEffect {
    pub effect: Effect,
    pub source: Entity,
    pub target: Entity,
}

struct PerformMomentaryEffect {
    pub momentary_effect: MomentaryEffect,
    pub source: Entity,
    pub target: Entity,
}

pub struct PeriodicMomentaryEffects {
    instances: Vec<PeriodicMomentaryEffectInstance>,
}

impl PeriodicMomentaryEffects {
    pub fn new() -> Self {
        Self { instances: vec![] }
    }
}

struct PeriodicMomentaryEffectInstance {
    momentary_effect: MomentaryEffect,
    interval_timer: Timer,
    duration_timer: Timer,
    source: Entity,
}

pub struct LastingEffects {
    pub instances: Vec<LastingEffectInstance>,
}

impl LastingEffects {
    pub fn new() -> Self {
        Self { instances: vec![] }
    }
}

pub struct LastingEffectInstance {
    pub lasting_effect: LastingEffect,
    pub duration_timer: Timer,
    pub source: Entity,
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PerformEffect>()
            .add_event::<PerformMomentaryEffect>()
            .add_system(perform_effect.system())
            .add_system(perform_momentary_effect.system())
            .add_system(tick_periodic_momentary_effects.system())
            .add_system(tick_lasting_effects.system());
    }
}

fn perform_effect(
    mut perform_effect_event_reader: EventReader<PerformEffect>,
    mut perform_momentary_effect_event_writer: EventWriter<PerformMomentaryEffect>,
    mut periodic_momentary_effects_query: Query<&mut PeriodicMomentaryEffects>,
    mut lasting_effects_query: Query<&mut LastingEffects>,
) {
    for perform_effect in perform_effect_event_reader.iter() {
        match perform_effect.effect {
            Effect::Momentary { momentary_effect } => {
                perform_momentary_effect_event_writer.send(PerformMomentaryEffect {
                    momentary_effect,
                    source: perform_effect.source,
                    target: perform_effect.target,
                })
            }
            Effect::PeriodicMomentary {
                momentary_effect,
                interval,
                duration,
            } => {
                let mut periodic_momentary_effects = periodic_momentary_effects_query
                    .get_mut(perform_effect.target)
                    .unwrap();
                periodic_momentary_effects
                    .instances
                    .push(PeriodicMomentaryEffectInstance {
                        momentary_effect,
                        interval_timer: Timer::from_seconds(interval, true),
                        duration_timer: Timer::from_seconds(duration, false),
                        source: perform_effect.source,
                    })
            }
            Effect::Lasting {
                lasting_effect,
                duration,
            } => {
                let mut lasting_effects = lasting_effects_query
                    .get_mut(perform_effect.target)
                    .unwrap();
                lasting_effects.instances.push(LastingEffectInstance {
                    lasting_effect,
                    duration_timer: Timer::from_seconds(duration, false),
                    source: perform_effect.source,
                })
            }
        }
    }
}

fn perform_momentary_effect(
    mut perform_momentary_effect_event_reader: EventReader<PerformMomentaryEffect>,
    mut critical_query: Query<Option<&Critical>>,
    mut health_query: Query<&mut Health>,
) {
    let mut rng = rand::thread_rng();

    for perform_momentary_effect in perform_momentary_effect_event_reader.iter() {
        let target = perform_momentary_effect.target;

        match perform_momentary_effect.momentary_effect {
            MomentaryEffect::LoseHealth {
                min_points,
                max_points,
            } => {
                let mut health = health_query.get_mut(target).unwrap();
                let mut points = rng.gen_range(min_points..=max_points);

                let critical = critical_query
                    .get_mut(perform_momentary_effect.source)
                    .unwrap();
                if let Some(critical) = critical {
                    if critical.percent >= rng.gen() {
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
            MomentaryEffect::GainHealth {
                min_points,
                max_points,
            } => {
                let mut health = health_query.get_mut(target).unwrap();
                let mut points = rng.gen_range(min_points..=max_points);

                let critical = critical_query
                    .get_mut(perform_momentary_effect.source)
                    .unwrap();
                if let Some(critical) = critical {
                    if critical.percent >= rng.gen() {
                        points *= CRITICAL_MULTIPLIER;
                    }
                }

                health.points = (health.points + points).min(health.max_points);
            }
        }
    }
}

fn tick_periodic_momentary_effects(
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
                    momentary_effect: instance.momentary_effect,
                    source: instance.source,
                    target: entity,
                });
            }

            instance.duration_timer.tick(time.delta());
        }

        instances.retain(|instance| !instance.duration_timer.finished());
    }
}

fn tick_lasting_effects(time: Res<Time>, mut query: Query<&mut LastingEffects>) {
    for mut lasting_effects in query.iter_mut() {
        let instances = &mut lasting_effects.instances;
        for instance in instances.iter_mut() {
            instance.duration_timer.tick(time.delta());
        }

        instances.retain(|instance| !instance.duration_timer.finished());
    }
}
