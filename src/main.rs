use bevy::prelude::*;

fn hello_world() {
    println!("Hello World!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Oliver Ellis".to_string())));
    commands.spawn((Person, Name("Patrick Dehning".to_string())));
    commands.spawn((Person, Name("Arne Eckel".to_string())));
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world)
        .run();
}
