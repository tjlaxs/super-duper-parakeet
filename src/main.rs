use bevy::prelude::*;

const PARAKEET_SIZE: Vec2 = Vec2::new(60.0, 60.0);
const PARAKEET_SPEED: f32 = 400.0;

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

fn move_parakeet(
    input: Res<ButtonInput<KeyCode>>,
    mut parakeet: Single<&mut Transform, With<Parakeet>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if input.pressed(KeyCode::KeyH) {
        direction -= 1.0;
    }

    if input.pressed(KeyCode::KeyL) {
        direction += 1.0;
    }

    parakeet.translation.x += direction * PARAKEET_SPEED * time.delta_secs();
}

fn basic_keys(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_parakeet, basic_keys))
        .run();
}
