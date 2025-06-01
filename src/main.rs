use bevy::prelude::*;

fn setup() {
    println!("setup")
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
