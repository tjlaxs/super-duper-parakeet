use bevy::prelude::*;

mod parakeet;
use parakeet::ParakeetPlugin;

const PARAKEET_SIZE: Vec2 = Vec2::new(60.0, 60.0);
const PARAKEET_SPEED: f32 = 400.0;
const INVADER_SPEED: f32 = 100.0;
const INVADER_ROWS: u32 = 3;
const INVADER_COLS: u32 = 7;

#[derive(Clone, Copy)]
enum HorizontalMovement {
    Left = -1,
    Right = 1,
}

#[derive(Resource)]
struct InvaderMovement(HorizontalMovement);

#[derive(Component)]
struct Invader;

fn setup(mut commands: Commands, windows: Query<&mut Window>) {
    let window = windows.single().unwrap();
    let width = window.resolution.width();
    let height = window.resolution.height();
    commands.insert_resource(InvaderMovement(HorizontalMovement::Left));
    commands.spawn(Camera2d);
    for row in 0..INVADER_ROWS {
        for col in 0..INVADER_COLS {
            let (x, y) = invader_position(width, height, col, row);
            commands.spawn((
                Invader,
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

fn move_invader(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Invader>>,
    mut invader_movement: ResMut<InvaderMovement>,
    windows: Query<&mut Window>,
) {
    let window = windows.single().unwrap();
    let w_max = (window.resolution.width() - PARAKEET_SIZE.x) / 2.0;

    let direction = invader_movement.0 as i32 as f32;
    let mut change_direction = false;
    query.iter_mut().for_each(|mut t| {
        t.translation.x += direction * INVADER_SPEED * time.delta_secs();
        if t.translation.x < -w_max || t.translation.x > w_max {
            change_direction = true;
        }
    });
    if change_direction {
        match invader_movement.0 {
            HorizontalMovement::Left => invader_movement.0 = HorizontalMovement::Right,
            HorizontalMovement::Right => invader_movement.0 = HorizontalMovement::Left,
        };
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
        .add_plugins(ParakeetPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_invader, basic_keys))
        .run();
}
