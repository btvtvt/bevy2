use bevy::prelude::*;

fn main() {
    println!("Hello, Bevy!");

    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("hello world!");
}