use crate::{PARAKEET_SIZE, PARAKEET_SPEED};
use bevy::prelude::*;

#[derive(Component)]
pub struct Parakeet;

pub struct ParakeetPlugin;

impl Plugin for ParakeetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, move_parakeet);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Parakeet,
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -300.0, 0.0),
            scale: PARAKEET_SIZE.extend(1.0),
            ..default()
        },
    ));
}

pub fn move_parakeet(
    input: Res<ButtonInput<KeyCode>>,
    mut parakeet: Single<&mut Transform, With<Parakeet>>,
    windows: Query<&mut Window>,
    time: Res<Time>,
) {
    let window = windows.single().unwrap();
    let w_max = (window.resolution.width() - PARAKEET_SIZE.x) / 2.0;
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
