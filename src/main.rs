use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const TILE_SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_tiles))
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::FUCHSIA,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(50.0, 50.0, 0.0)),
            ..default()
        },
        Player,
    ));
}

fn spawn_tiles(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    let (width, height) = (window.width(), window.height());

    let horizontal_tile_count = ((width / TILE_SIZE).floor() + 1.) as i32;
    let vertical_tile_count = ((height / TILE_SIZE).floor() + 1.) as i32;

    for row in -vertical_tile_count / 2..vertical_tile_count / 2 {
        for column in -horizontal_tile_count / 2..horizontal_tile_count / 2 {
            let color = Color::rgb(rand::random(), rand::random(), rand::random());

            commands.spawn(SpriteBundle {
                sprite: Sprite { color, ..default() },
                transform: Transform {
                    translation: Vec3::new(column as f32 * TILE_SIZE, row as f32 * TILE_SIZE, -1.0),
                    rotation:    Quat::default(),
                    scale:       Vec3::new(TILE_SIZE, TILE_SIZE, 0.0),
                },
                ..default()
            });
        }
    }
}

const PLAYER_SPEED: f32 = 200.;

fn sprite_movement(
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut transform) = player_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}
