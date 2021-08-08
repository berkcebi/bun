use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 512.0;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Bun".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(bevy::core::FixedTimestep::step(1.0))
                .with_system(regen_mana.system()),
        )
        .add_system(handle_keyboard_input.system())
        .add_system(use_ability.system())
        .run();
}

struct Ability {
    name: String,
    mana_points: u8,
}

struct Player;

struct Mana {
    points: u8,
    max_points: u8,
    regen_points: u8,
}

struct UseAbility {
    ability: Ability,
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Player).insert(Mana {
        points: 50,
        max_points: 100,
        regen_points: 3,
    });
}

fn handle_keyboard_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_entity = player_query.single().unwrap();

    if keyboard_input.just_pressed(KeyCode::Key1) {
        commands.entity(player_entity).insert(UseAbility {
            ability: Ability {
                name: "Fireball".to_string(),
                mana_points: 25,
            },
        });
    }
}

fn regen_mana(mut query: Query<&mut Mana>) {
    for mut mana in query.iter_mut() {
        if mana.points < mana.max_points {
            mana.points = (mana.points + mana.regen_points).min(mana.max_points);
        }

        println!("{} / {}", mana.points, mana.max_points);
    }
}

fn use_ability(mut commands: Commands, mut query: Query<(Entity, &UseAbility, &mut Mana)>) {
    for (entity, use_ability_intent, mut mana) in query.iter_mut() {
        if use_ability_intent.ability.mana_points < mana.points {
            mana.points -= use_ability_intent.ability.mana_points;
            println!("Casting {}!", use_ability_intent.ability.name);
        } else {
            println!("Not enough mana…");
        }

        commands.entity(entity).remove::<UseAbility>();
    }
}
