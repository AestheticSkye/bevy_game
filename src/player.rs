//! Player movement and animation.

mod coord_display;
mod walk_animation;

use bevy::prelude::*;
use bevy::sprite::Anchor;

use self::coord_display::{setup_coords, update_coords};
use self::walk_animation::{walk_animation, WalkAnimator};
use crate::get_single_mut;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_coords)).add_systems(
            Update,
            (sprite_movement, walk_animation, update_coords).chain(),
        );
    }
}

const PLAYER_SPEED: f32 = 200.;

#[derive(Component, Default)]
pub struct Player {
    facing_direction: Direction,
    /// Stores the state of players [Sprites](Sprite) walk animation.
    /// Should be [None] if the player is not moving
    walk_animator:    Option<WalkAnimator>,
}

impl Player {
    /// Starts a timer for walking animation if one doesnt exist already.
    fn start_walk_animation(&mut self, inverted: bool) {
        let direction = if inverted {
            self.facing_direction.next()
        } else {
            self.facing_direction
        };

        if self.walk_animator.is_none() {
            self.walk_animator = Some(WalkAnimator::new(direction));
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
    const fn next(self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Left => Self::Right,
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
    let (mut player_transform, mut sprite, mut player) = get_single_mut!(player_query);
    let mut camera_transform = get_single_mut!(camera_query);

    let mut direction = Vec3::ZERO;

    for key_press in keyboard_input.get_pressed() {
        match key_press {
            KeyCode::ArrowLeft | KeyCode::KeyA => {
                direction.x += -1.0;
                player.facing_direction = Direction::Left;
                player.start_walk_animation(false);
            }
            KeyCode::ArrowRight | KeyCode::KeyD => {
                direction.x += 1.0;
                player.facing_direction = Direction::Right;
                player.start_walk_animation(false);
            }
            KeyCode::ArrowUp | KeyCode::KeyW => {
                direction.y += 1.0;
                player.start_walk_animation(false);
            }
            KeyCode::ArrowDown | KeyCode::KeyS => {
                direction.y += -1.0;
                player.start_walk_animation(true);
            }
            _ => {}
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
