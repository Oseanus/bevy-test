use bevy::prelude::*;

fn hello_world() {
    println!("Hello World!");
}

// A component for a person
#[derive(Component)]
struct Person;

// A component for a name in order to use it more flexibly (e.g workss for pets as well)
#[derive(Component)]
struct Name(String);

// System that creates perple at startup
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Oliver Ellis".to_string())));
    commands.spawn((Person, Name("Patrick Dehning".to_string())));
    commands.spawn((Person, Name("Arne Eckel".to_string())));
}

// Define a resource
#[derive(Resource)]
struct GreetTimer(Timer);

// A system to query people created at startup schedule
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}", name.0);
        }
    }
}

// A mutable system for changing attributes of certain components
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Arne Eckel" {
            name.0 = "Arne Eckel-Dehning".to_string();
            break; // We don’t need to change any other names
        }
    }
}

// A custom plugin for bevy
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // Systems are usually are running parallel per scheduling level but sets of systems can be chained to a single task
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, (update_people, greet_people).chain()));
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
