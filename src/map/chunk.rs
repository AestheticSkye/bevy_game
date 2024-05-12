use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use strum::IntoEnumIterator;
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{self, NoiseMapGeneratorBase};

use super::chunk_position::ChunkPosition;
use super::config::MapConfig;
use super::tile_kind::TileKind;
use super::{ChunkBorderState, Map};

#[derive(Component, Deref, DerefMut)]
pub struct Chunk(pub Vec<Vec<TileKind>>);

impl Chunk {
    /// Generates a [`Chunk`] from the noisemap for a given position.
    pub fn new(
        position: ChunkPosition,
        noisemap: &noisemap::NoiseMap<PerlinNoise>,
        config: &MapConfig,
    ) -> Self {
        let chunk_noise = noisemap.generate_chunk(i64::from(position.x), i64::from(position.y));

        let mut tiles =
            vec![vec![TileKind::Grass; config.chunk_tile_count]; config.chunk_tile_count];

        for (row_index, row) in chunk_noise.iter().enumerate() {
            for (tile_index, tile) in row.iter().enumerate() {
                let tile_type = tile.into();

                tiles[row_index][tile_index] = tile_type;
            }
        }

        Self(tiles)
    }

    /// Convert a [`Chunk`] and its data into a bevy
    /// [`Image`](https://docs.rs/bevy/latest/bevy/render/texture/struct.Image.html) to be used for creating textures.
    ///
    /// Image is still required to be registered in [`Assets<Image>`] to be used as a texture.
    pub fn generate_texture_image(
        &self,
        config: &MapConfig,
        chunk_borders: ChunkBorderState,
    ) -> Image {
        let chunk_size = config.chunk_size() as u32;
        let tile_size = config.tile_size as u32;

        let mut dyn_image = DynamicImage::new_rgb8(chunk_size, chunk_size);

        // Fill image completely with one colour if all tiles are the same, otherwise draw the tiles.
        if let Some(tile_type) = self.is_uniform_type() {
            let color: Color = tile_type.into();
            draw_filled_rect_mut(
                &mut dyn_image,
                Rect::at(0, 0).of_size(chunk_size, chunk_size),
                Rgba(color.as_rgba_u8()),
            );
        } else {
            for (row_index, row) in self.0.iter().rev().enumerate() {
                let row_index = row_index as u32;

                for (tile_index, tile_type) in row.iter().enumerate() {
                    let tile_index = tile_index as u32;
                    let color: Color = tile_type.into();

                    draw_filled_rect_mut(
                        &mut dyn_image,
                        Rect::at(
                            (tile_index * tile_size) as i32,
                            (row_index * tile_size) as i32,
                        )
                        .of_size(tile_size, tile_size),
                        Rgba(color.as_rgba_u8()),
                    );
                }
            }
        }

        if chunk_borders == ChunkBorderState::Shown {
            Self::draw_chunk_border(&mut dyn_image, config);
        }

        Image::from_dynamic(dyn_image, true, RenderAssetUsages::RENDER_WORLD)
    }

    fn draw_chunk_border(chunk_image: &mut DynamicImage, config: &MapConfig) {
        let color = Color::GRAY;

        draw_filled_rect_mut(
            chunk_image,
            Rect::at(0, 0).of_size(5, config.chunk_size() as u32),
            Rgba(color.as_rgba_u8()),
        );
        draw_filled_rect_mut(
            chunk_image,
            Rect::at(0, 0).of_size(config.chunk_size() as u32, 5),
            Rgba(color.as_rgba_u8()),
        );
    }

    /// Spawn & render a chunk with its given texture to the games map.
    /// `texture` must correspond to `chunk`.
    pub fn render(
        self,
        commands: &mut Commands,
        map: &mut Map,
        config: &MapConfig,
        texture: Handle<Image>,
        position: ChunkPosition,
    ) {
        // Where in the map to to start rendering the chunk, based `position`
        let horizontal_shift = config.chunk_size() * (position.x as f32 - 1.0);
        let horizontal_start_pos =
            (config.chunk_tile_count as f32 / 2.).mul_add(config.tile_size, horizontal_shift);
        let vertical_shift = config.chunk_size() * (position.y as f32 - 1.0);
        let vertical_start_pos =
            (config.chunk_tile_count as f32 / 2.).mul_add(config.tile_size, vertical_shift);

        let chunk_id = commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(Vec3 {
                        x: horizontal_start_pos,
                        y: vertical_start_pos,
                        z: -1.,
                    }),
                    ..default()
                },
                self,
                position,
            ))
            .id();

        map.insert(position, chunk_id);
    }

    /// Returns the tile type of all of the tiles if the chunk only contains one tile type.
    /// i.e. All ocean.
    fn is_uniform_type(&self) -> Option<TileKind> {
        TileKind::iter().find(|&tile_type| {
            !self
                .iter()
                .any(|row| row.iter().any(|tile| *tile != tile_type))
        })
    }
}
