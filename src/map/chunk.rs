use bevy::ecs::component::Component;
use strum::IntoEnumIterator;

use super::tile_type::TileType;
use super::CHUNK_TILE_COUNT;

#[derive(Component)]
pub struct Chunk(pub [[TileType; CHUNK_TILE_COUNT]; CHUNK_TILE_COUNT]);

impl Chunk {
    /// Returns the tile type of all of the tiles if the chunk only contains one tile type.
    /// i.e. All ocean.
    pub fn is_uniform_type(&self) -> Option<TileType> {
        TileType::iter().find(|&tile_type| {
            !self
                .0
                .iter()
                .any(|row| row.iter().any(|tile| *tile != tile_type))
        })
    }
}
