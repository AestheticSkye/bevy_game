mod coord_display;
mod tilt;

use bevy::prelude::*;
use bevy::sprite::Anchor;

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

impl Player {
    /// Starts a timer for walking animation if one doesnt exist already.
    fn start_timer(&mut self, direction: Direction) {
        if self.tilt.is_none() {
            self.tilt = Some(TiltTimer::new(direction));
        }
    }
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
    // Makes the ancor just above the feet, for the walking animation.
    let anchor = Anchor::Custom(Vec2 { x: 0.0, y: -0.2 });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                anchor,
                ..default()
            },
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
        direction += Vec3::new(-1.0, 0.0, 0.0);

        player.facing_direction = Direction::Left;
        // The direction the player is facing, also used as the initial tilt direction except for when going down.
        // This is to make the animation look uniform when it starts. Has to be declared seperately cus oWnErShIp
        let facing_direction = player.facing_direction;
        player.start_timer(facing_direction);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        direction += Vec3::new(1.0, 0.0, 0.0);

        player.facing_direction = Direction::Right;
        let facing_direction = player.facing_direction;
        player.start_timer(facing_direction);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        direction += Vec3::new(0.0, 1.0, 0.0);

        let facing_direction = player.facing_direction;
        player.start_timer(facing_direction);
    }
    if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        direction += Vec3::new(0.0, -1.0, 0.0);

        let facing_direction = player.facing_direction.next();
        player.start_timer(facing_direction);
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
