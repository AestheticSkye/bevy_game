mod coord_display;
mod tilt;

use bevy::prelude::*;

use self::coord_display::{setup_coords, update_coords};
use self::tilt::{tilt_sprite, TiltTimer};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_coords)).add_systems(
            Update,
            (sprite_movement, tilt_sprite, update_coords).chain(),
        );
    }
}

const PLAYER_SPEED: f32 = 200.;

#[derive(Component, Default)]
pub struct Player {
    facing_direction: Direction,
    tilt:             Option<TiltTimer>,
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Right,
    Left,
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            // sprite: Sprite {
            //     color: Color::FUCHSIA,
            //     ..default()
            // },
            texture: asset_server.load("sprites/honse.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(5.0, 5.0, 0.0)),
            ..default()
        },
        Player::default(),
    ));
}

pub fn sprite_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Sprite, &mut Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok((mut player_transform, mut sprite, mut player)) = player_query.get_single_mut() else {
        return;
    };

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        player.facing_direction = Direction::Left;
        if player.tilt.is_none() {
            player.tilt = Some(TiltTimer::new(Direction::Left, 0.0));
        }
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        player.facing_direction = Direction::Right;
        if player.tilt.is_none() {
            player.tilt = Some(TiltTimer::new(Direction::Right, 0.0));
        }
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction += Vec3::new(0.0, 1.0, 0.0);
        if player.tilt.is_none() {
            player.tilt = Some(TiltTimer::new(Direction::Right, 0.0));
        }
    }
    if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction += Vec3::new(0.0, -1.0, 0.0);
        if player.tilt.is_none() {
            player.tilt = Some(TiltTimer::new(Direction::Right, 0.0));
        }
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    camera_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    match player.facing_direction {
        Direction::Left => sprite.flip_x = true,
        Direction::Right => sprite.flip_x = false,
    }
}
