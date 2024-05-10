use std::time::Duration;

use bevy::prelude::*;

use super::{Direction, Player};
use crate::get_single_mut;

const TILT_SPEED: f32 = 0.25;
const TILT_RADIUS: f32 = 0.25;

#[derive(Component)]
pub struct TiltTimer {
    timer:     Timer,
    direction: Direction,
}

impl TiltTimer {
    pub fn new(direction: Direction) -> Self {
        TiltTimer {
            timer: Timer::from_seconds(0.0, TimerMode::Repeating),
            direction,
        }
    }
}

pub fn tilt_sprite(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut player_transform, mut player) = get_single_mut!(player_query);

    let Some(tilt) = &mut player.tilt else {
        return;
    };

    tilt.timer.tick(time.delta());

    if tilt.timer.finished() {
        let mut transform = Transform::default();
        match tilt.direction {
            Direction::Right => transform.rotate_z(TILT_RADIUS),
            Direction::Left => transform.rotate_z(-TILT_RADIUS),
        }
        player_transform.rotation = transform.rotation;
        // Make sure the timer has a proper timings after
        tilt.timer.set_duration(Duration::from_secs_f32(TILT_SPEED));
        tilt.direction = tilt.direction.next();
    }

    if !keyboard_input.any_pressed([
        KeyCode::ArrowRight,
        KeyCode::KeyD,
        KeyCode::ArrowLeft,
        KeyCode::KeyA,
        KeyCode::ArrowUp,
        KeyCode::KeyW,
        KeyCode::ArrowDown,
        KeyCode::KeyS,
    ]) {
        player_transform.rotation = Quat::default();
        player.tilt = None;
    }
}
