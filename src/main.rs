use bevy::prelude::*;

fn main() {
    App::build()
        // All Bevy engine features are implemented as plugins
        // adds an "event loop" to our application. Our App's 
        // ECS Schedule now runs in a loop once per "frame"
        .add_default_plugins() // WindowPlugin, WinitPlugin
        .add_plugin(HelloPlugin)
        // Startup systems are just like normal systems, but they run exactly once, 
        //before all other systems, right when our app starts. 
        // .add_startup_system(add_people.system())
        // // add_system() function adds the system to App's Schedule
        // .add_system(hello_world.system())
        // // The parameters we pass in to a "system function" define what entities the system runs on. In this case, 
        // // greet_people will run on all entities with the Person and Name component.
        // .add_system(greet_people.system())
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

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    // multi ecs -> plugin 
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(add_people.system())
        // add_system() function adds the system to App's Schedule
        // .add_system(hello_world.system())
        // The parameters we pass in to a "system function" define what entities the system runs on. In this case, 
        // greet_people will run on all entities with the Person and Name component.
        .add_system(greet_people.system());
    }
}
