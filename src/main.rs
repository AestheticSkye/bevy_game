mod map;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use map::MapPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter:            "info,wgpu_core=warn,wgpu_hal=warn,bevy_game=debug".into(),
                level:             bevy::log::Level::DEBUG,
                update_subscriber: None,
            }),
            MapPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
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
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player_transform) = player_query.get_single_mut() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
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

    camera_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
}
