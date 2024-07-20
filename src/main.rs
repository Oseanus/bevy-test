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

// A system to query people created at startup schedule
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0);
    }
}

// A mutable system for changing attibutes of certain components
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Arne Eckel" {
            name.0 = "Arne Eckel-Dehning".to_string();
            break; // We don’t need to change any other names
        }
    }
}

fn main() {
    App::new()
        // Systems are usually running parallel per scheduling level but sets of systems can be chained to a single task
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
