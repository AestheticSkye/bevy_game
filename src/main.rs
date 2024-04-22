mod map;

use bevy::prelude::*;
use map::MapPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapPlugin))
        .add_systems(Startup, setup)
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
