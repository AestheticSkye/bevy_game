use bevy::ecs::component::Component;

use super::CHUNK_SIZE;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Position {
            x: (value.0 / CHUNK_SIZE) as i32,
            y: (value.1 / CHUNK_SIZE) as i32,
        }
    }
}
