use bevy::prelude::*;

const PARAKEET_SIZE: Vec2 = Vec2::new(60.0, 60.0);
const PARAKEET_SPEED: f32 = 400.0;
const INVADER_ROWS: u32 = 3;
const INVADER_COLS: u32 = 7;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Parakeet;

#[derive(Component)]
struct Invader;

fn setup(mut commands: Commands, windows: Query<&mut Window>) {
    let window = windows.single().unwrap();
    let width = window.resolution.width();
    let height = window.resolution.height();
    commands.spawn(Camera2d);
    commands.spawn((
        Parakeet,
        Collider,
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            scale: PARAKEET_SIZE.extend(1.0),
            ..default()
        },
    ));
    for row in 0..INVADER_ROWS {
        for col in 0..INVADER_COLS {
            let (x, y) = invader_position(width, height, col, row);
            commands.spawn((
                Invader,
                Collider,
                Sprite::from_color(Color::srgb(0.1, 0.9, 0.15), Vec2::ONE),
                Transform {
                    translation: Vec3::new(x, 400.0 - y, 0.0),
                    scale: PARAKEET_SIZE.extend(1.0),
                    ..default()
                },
            ));
        }
    }
}

fn invader_position(width: f32, height: f32, col: u32, row: u32) -> (f32, f32) {
    let x = (col as f32 + 1.0) * width / (INVADER_COLS as f32 + 1.0) - width / 2.0;
    let y = (row as f32 + 1.0) * height / (2.0 * INVADER_ROWS as f32 + 1.0);
    (x, y)
}

fn move_parakeet(
    input: Res<ButtonInput<KeyCode>>,
    mut parakeet: Single<&mut Transform, With<Parakeet>>,
    windows: Query<&mut Window>,
    time: Res<Time>,
) {
    let window = windows.single().unwrap();
    let w_max = window.resolution.width() / 2.0;
    let mut direction = 0.0;

    if input.pressed(KeyCode::KeyH) {
        direction -= 1.0;
    }

    if input.pressed(KeyCode::KeyL) {
        direction += 1.0;
    }

    let new_x = parakeet.translation.x + direction * PARAKEET_SPEED * time.delta_secs();
    if new_x < -w_max {
        parakeet.translation.x = -w_max;
    } else if new_x > w_max {
        parakeet.translation.x = w_max;
    } else {
        parakeet.translation.x = new_x;
    }
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
