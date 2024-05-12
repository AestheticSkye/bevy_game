use bevy::ecs::component::Component;

use super::config::MapConfig;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

impl ChunkPosition {
    pub fn from_xy(value: (f32, f32), map_config: &MapConfig) -> Self {
        Self {
            x: (value.0 / map_config.chunk_size()) as i32,
            y: (value.1 / map_config.chunk_size()) as i32,
        }
    }
}
