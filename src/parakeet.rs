use crate::bullet::Bullet;
use crate::{BULLET_SIZE, PARAKEET_SIZE, PARAKEET_SPEED};
use bevy::prelude::*;

#[derive(Component)]
struct Parakeet;

pub struct ParakeetPlugin;

impl Plugin for ParakeetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (bullet_time, shoot).chain())
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

fn shoot(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    parakeet: Single<&mut Transform, With<Parakeet>>,
) {
    if input.just_pressed(KeyCode::KeyF) {
        commands.spawn((
            Bullet(200.),
            Transform::from_translation(parakeet.translation).with_scale(BULLET_SIZE.extend(1.0)),
            Sprite::from_color(Color::hsl(50., 0.5, 0.5), Vec2::ONE),
        ));
    }
}

fn bullet_time(
    mut commands: Commands,
    bullets: Query<(Entity, &Bullet, &mut Transform)>,
    windows: Query<&mut Window>,
    time: Res<Time>,
) {
    let window = windows.single().unwrap();
    let h_max = (window.resolution.height() + PARAKEET_SIZE.y) / 2.;
    for (e, Bullet(dir), mut t) in bullets {
        if t.translation.y > h_max {
            commands.entity(e).despawn();
            break;
        }
        t.translation.y += dir * time.delta_secs();
    }
}

fn move_parakeet(
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
