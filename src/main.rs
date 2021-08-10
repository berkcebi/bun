use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 512.0;

const USE_ABILITY_COOLDOWN_DURATION: f32 = 1.5;

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
        .add_system(remove_use_ability_cooldown.system())
        .run();
}

#[derive(Debug, Clone, Copy)]
struct Ability {
    name: &'static str,
    mana_points: u8,
    use_duration: f32,
}

impl Ability {
    const FIREBALL: Self = Self {
        name: "Fireball",
        mana_points: 25,
        use_duration: 2.5,
    };

    const FIRE_BLAST: Self = Self {
        name: "Fire Blast",
        mana_points: 10,
        use_duration: 0.0,
    };
}

struct Player;

struct Mana {
    points: u8,
    max_points: u8,
    regen_points: u8,
}

struct UseAbility {
    ability: Ability,
    duration_timer: Timer,
}

impl UseAbility {
    fn new(ability: Ability) -> Self {
        Self {
            ability,
            duration_timer: Timer::from_seconds(ability.use_duration, false),
        }
    }
}

struct UseAbilityCooldown {
    duration_timer: Timer,
}

impl UseAbilityCooldown {
    fn new() -> Self {
        Self {
            duration_timer: Timer::from_seconds(USE_ABILITY_COOLDOWN_DURATION, false),
        }
    }
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
        commands
            .entity(player_entity)
            .insert(UseAbility::new(Ability::FIREBALL));
    }

    if keyboard_input.just_pressed(KeyCode::Key2) {
        commands
            .entity(player_entity)
            .insert(UseAbility::new(Ability::FIRE_BLAST));
    }
}

fn regen_mana(mut query: Query<&mut Mana>) {
    for mut mana in query.iter_mut() {
        if mana.points < mana.max_points {
            mana.points = (mana.points + mana.regen_points).min(mana.max_points);
        }

        println!("Mana: {} / {}", mana.points, mana.max_points);
    }
}

fn use_ability(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut UseAbility,
        &mut Mana,
        Option<&UseAbilityCooldown>,
    )>,
) {
    for (entity, mut use_ability, mut mana, use_ability_cooldown) in query.iter_mut() {
        let ability = use_ability.ability;

        if use_ability.duration_timer.elapsed_secs() <= 0.0 {
            if use_ability_cooldown.is_some() {
                println!("Under global cooldown.");

                commands.entity(entity).remove::<UseAbility>();
                continue;
            }

            if ability.mana_points > mana.points {
                println!("Not enough mana.");

                commands.entity(entity).remove::<UseAbility>();
                continue;
            }

            println!("Casting {}…", ability.name);

            commands.entity(entity).insert(UseAbilityCooldown::new());
        }

        use_ability.duration_timer.tick(time.delta());

        if use_ability.duration_timer.finished() {
            mana.points -= ability.mana_points;

            println!("Casted {}!", ability.name);
            println!("Mana: {} / {}", mana.points, mana.max_points);

            commands.entity(entity).remove::<UseAbility>();
        }
    }
}

fn remove_use_ability_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut UseAbilityCooldown)>,
) {
    for (entity, mut use_ability_cooldown) in query.iter_mut() {
        use_ability_cooldown.duration_timer.tick(time.delta());

        if use_ability_cooldown.duration_timer.finished() {
            println!("Global cooldown over.");
            commands.entity(entity).remove::<UseAbilityCooldown>();
        }
    }
}
