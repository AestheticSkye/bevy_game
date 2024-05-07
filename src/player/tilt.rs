use bevy::prelude::*;

use super::{Direction, Player};

const TILT_SPEED: f32 = 0.25;
const TILT_RADIUS: f32 = 0.25;

#[derive(Component)]
pub struct TiltTimer {
    timer:     Timer,
    direction: Direction,
}

impl TiltTimer {
    pub fn new(direction: Direction, duration: f32) -> Self {
        TiltTimer {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
            direction,
        }
    }
}

pub fn tilt_sprite(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    let Ok((mut player_transform, mut player)) = player_query.get_single_mut() else {
        return;
    };

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
        *tilt = TiltTimer::new(tilt.direction.next(), TILT_SPEED)
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
