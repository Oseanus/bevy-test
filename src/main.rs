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

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}
