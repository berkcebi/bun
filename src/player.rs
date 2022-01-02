use crate::{
    ability::{Ability, TryAbility},
    creature::{CreatureBundle, CREATURE_SPEED},
    effect::{Effect, LastingEffect, MomentaryEffect, MomentaryEffectSchedule},
    sprite::Sprite,
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

fn spawn(mut commands: Commands, texture_atlases: Res<Assets<TextureAtlas>>) {
    commands
        .spawn_bundle(CreatureBundle::new(160, 100))
        .insert(Player)
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.get_handle(Sprite::SHEET_PATH),
            sprite: TextureAtlasSprite::new(Sprite::Player.index()),
            ..Default::default()
        });
}

fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut try_ability_event_writer: EventWriter<TryAbility>,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
) {
    let (player_entity, mut player_transform) = player_query.single_mut().unwrap();

    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }

    player_transform.translation += direction * time.delta_seconds() * CREATURE_SPEED;

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
