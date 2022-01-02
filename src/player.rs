use crate::{
    ability::{Ability, TryAbility},
    critical::Critical,
    effect::{
        Effect, LastingEffect, LastingEffects, MomentaryEffect, MomentaryEffectSchedule,
        PeriodicMomentaryEffects,
    },
    health::Health,
    mana::Mana,
};
use bevy::prelude::*;

pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn.system())
            .add_system(handle_keyboard_input.system());
    }
}

fn spawn(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Health {
            points: 160,
            max_points: 200,
        })
        .insert(Mana {
            points: 100,
            max_points: 100,
            regen_points: 1,
        })
        .insert(Critical { percent: 0.1 })
        .insert(PeriodicMomentaryEffects::default())
        .insert(LastingEffects::default());
}

fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut try_ability_event_writer: EventWriter<TryAbility>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_entity = player_query.single().unwrap();

    if keyboard_input.just_pressed(KeyCode::Key1) {
        try_ability_event_writer.send(TryAbility {
            source: player_entity,
            ability: Ability {
                name: "Fireball",
                mana_points: 20,
                cast_duration: 2.5,
                effect: Effect::Momentary(
                    MomentaryEffect::LoseHealth(30, 50),
                    MomentaryEffectSchedule::Once,
                ),
                secondary_effect: Some(Effect::Momentary(
                    MomentaryEffect::LoseHealth(2, 3),
                    MomentaryEffectSchedule::Periodic(3.0, 12.0),
                )),
            },
            target: player_entity,
        });
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        try_ability_event_writer.send(TryAbility {
            source: player_entity,
            ability: Ability {
                name: "Fire Blast",
                mana_points: 15,
                cast_duration: 0.0,
                effect: Effect::Momentary(
                    MomentaryEffect::LoseHealth(20, 30),
                    MomentaryEffectSchedule::Once,
                ),
                secondary_effect: None,
            },
            target: player_entity,
        });
    }

    if keyboard_input.just_pressed(KeyCode::Key3) {
        try_ability_event_writer.send(TryAbility {
            source: player_entity,
            ability: Ability {
                name: "Lesser Heal",
                mana_points: 15,
                cast_duration: 1.5,
                effect: Effect::Momentary(
                    MomentaryEffect::GainHealth(40, 60),
                    MomentaryEffectSchedule::Once,
                ),
                secondary_effect: None,
            },
            target: player_entity,
        });
    }

    if keyboard_input.just_pressed(KeyCode::Q) {
        try_ability_event_writer.send(TryAbility {
            source: player_entity,
            ability: Ability {
                name: "Silence",
                mana_points: 20,
                cast_duration: 0.0,
                effect: Effect::Lasting(LastingEffect::Silence, 4.0),
                secondary_effect: None,
            },
            target: player_entity,
        });
    }
}
