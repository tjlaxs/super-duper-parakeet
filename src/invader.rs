use crate::{INVADER_COLS, INVADER_ROWS, INVADER_SPEED, PARAKEET_SIZE};
use bevy::prelude::*;

#[derive(Clone, Copy)]
enum HorizontalMovement {
    Left = -1,
    Right = 1,
}

#[derive(Resource)]
struct InvaderMovement(HorizontalMovement);

#[derive(Component)]
struct Invader;

pub struct InvaderPlugin;

impl Plugin for InvaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_invader);
    }
}

fn invader_position(width: f32, height: f32, col: u32, row: u32) -> (f32, f32) {
    let x = (col as f32 + 1.0) * width / (INVADER_COLS as f32 + 1.0) - width / 2.0;
    let y = (row as f32 + 1.0) * height / (2.0 * INVADER_ROWS as f32 + 1.0);
    (x, y)
}

fn setup(mut commands: Commands, windows: Query<&mut Window>) {
    let window = windows.single().unwrap();
    let width = window.resolution.width();
    let height = window.resolution.height();
    commands.insert_resource(InvaderMovement(HorizontalMovement::Left));
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
