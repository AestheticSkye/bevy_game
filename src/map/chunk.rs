use bevy::ecs::component::Component;
use bevy::prelude::{Deref, DerefMut};
use strum::IntoEnumIterator;

use super::tile_kind::TileKind;

#[derive(Component, Deref, DerefMut)]
pub struct Chunk(pub Vec<Vec<TileKind>>);

impl Chunk {
    /// Returns the tile type of all of the tiles if the chunk only contains one tile type.
    /// i.e. All ocean.
    pub fn is_uniform_type(&self) -> Option<TileKind> {
        TileKind::iter().find(|&tile_type| {
            !self
                .iter()
                .any(|row| row.iter().any(|tile| *tile != tile_type))
        })
    }
}
