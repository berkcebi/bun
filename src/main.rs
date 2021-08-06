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
        .run();
}

struct Mana {
    points: u8,
    max_points: u8,
    regen_points: u8,
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Mana {
        points: 50,
        max_points: 100,
        regen_points: 3,
    });
}

fn regen_mana(mut query: Query<&mut Mana>) {
    for mut mana in query.iter_mut() {
        if mana.points < mana.max_points {
            mana.points = (mana.points + mana.regen_points).min(mana.max_points);
        }

        println!("{} / {}", mana.points, mana.max_points);
    }
}
