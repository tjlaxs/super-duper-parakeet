use bevy::prelude::*;

mod bullet;

mod parakeet;
use parakeet::ParakeetPlugin;

mod invader;
use invader::InvaderPlugin;

const PARAKEET_SIZE: Vec2 = Vec2::new(60.0, 60.0);
const PARAKEET_SPEED: f32 = 400.0;
const INVADER_SPEED: f32 = 100.0;
const INVADER_ROWS: u32 = 3;
const INVADER_COLS: u32 = 7;
const BULLET_SIZE: Vec2 = Vec2::new(15.0, 15.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
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
        .add_plugins(InvaderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, basic_keys)
        .run();
}
