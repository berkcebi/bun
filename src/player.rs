use crate::{
    ability::{Ability, UseAbility},
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
            points: 30,
            max_points: 40,
        })
        .insert(Mana {
            points: 50,
            max_points: 100,
            regen_points: 1,
        });
}

fn handle_keyboard_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_entity = player_query.single().unwrap();

    if keyboard_input.just_pressed(KeyCode::Key1) {
        commands
            .entity(player_entity)
            .insert(UseAbility::new(Ability::FIREBALL, player_entity));
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        commands
            .entity(player_entity)
            .insert(UseAbility::new(Ability::FIRE_BLAST, player_entity));
    }

    if keyboard_input.just_pressed(KeyCode::Key3) {
        commands
            .entity(player_entity)
            .insert(UseAbility::new(Ability::LESSER_HEAL, player_entity));
    }
}
