use bevy::prelude::*;

const PARAKEET_SIZE: Vec2 = Vec2::new(60.0, 60.0);

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Parakeet;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Parakeet,
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            scale: PARAKEET_SIZE.extend(1.0),
            ..default()
        },
        Collider,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
