use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("John Snow".to_string()));
    commands.spawn().insert(Person).insert(Name("Sansa Stark".to_string()));
    commands.spawn().insert(Person).insert(Name("Arya Stark".to_string()));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
