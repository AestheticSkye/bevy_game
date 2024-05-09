use bevy::prelude::*;

const TILE_SIZE: f32 = 15.0;
const CHUNK_TILE_COUNT: usize = 20;

#[derive(Resource)]
pub struct MapConfig {
    /// Size of a tile in pixels.
    pub tile_size:        f32,
    /// The amount of tiles in a chunk.
    pub chunk_tile_count: usize,
}

impl MapConfig {
    /// Size of a chunk in pixels.
    pub fn chunk_size(&self) -> f32 { self.chunk_tile_count as f32 * self.tile_size }
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            tile_size:        TILE_SIZE,
            chunk_tile_count: CHUNK_TILE_COUNT,
        }
    }
}
