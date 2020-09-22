use bevy::prelude::*;

fn main() {
    App::build()
        // Startup systems are just like normal systems, but they run exactly once, 
        //before all other systems, right when our app starts. 
        .add_startup_system(add_people.system())
        // add_system() function adds the system to App's Schedule
        .add_system(hello_world.system())
        // The parameters we pass in to a "system function" define what entities the system runs on. In this case, 
        // greet_people will run on all entities with the Person and Name component.
        .add_system(greet_people.system())
         .run();
}

struct Person;
struct Name(String);

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("hello {}!", name.0);
}
