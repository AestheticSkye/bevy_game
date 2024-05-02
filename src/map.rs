mod chunk;
mod position;
mod tile_type;

use std::time::Instant;

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::{HashMap, HashSet};
use bevy::window::PrimaryWindow;
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{self, NoiseMapGenerator, NoiseMapGeneratorBase, Seed, Step};
use worldgen::world::Size;

use self::chunk::Chunk;
use self::position::Position;
use self::tile_type::TileType;
use crate::Player;

/// Size of a tile in pixels.
const TILE_SIZE: f32 = 15.0;
/// The amount of tiles in a chunk.
const CHUNK_TILE_COUNT: usize = 20;
/// Size of a chunk in pixels.
const CHUNK_SIZE: f32 = CHUNK_TILE_COUNT as f32 * TILE_SIZE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(generate_noise_map())
            .insert_resource(TileSet::default())
            .add_systems(Update, update_chunks);
    }
}

#[derive(Resource)]
struct NoiseMap(noisemap::NoiseMap<PerlinNoise>);

#[derive(Resource, Default)]
struct TileSet {
    spawned_chunks: HashMap<Position, Entity>,
}

fn generate_noise_map() -> NoiseMap {
    let noise = PerlinNoise::new();

    NoiseMap(
        noisemap::NoiseMap::new(noise)
        .set(Seed::of(rand::random::<i64>())) // Todo: Convert this into a propper seet system
        .set(Size::of(CHUNK_TILE_COUNT as i64, CHUNK_TILE_COUNT as i64))
        .set(Step::of(0.01, 0.01)),
    )
}

/// System to spawn and despawn the games chunks depending on camera placement.
fn update_chunks(
    mut commands: Commands,
    mut tileset: ResMut<TileSet>,
    mut assets: ResMut<Assets<Image>>,
    noisemap: Res<NoiseMap>,
    camera_transform: Query<&Transform, (With<Camera>, Without<Player>)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(camera_transform) = camera_transform.get_single() else {
        return;
    };

    let Ok(window) = window.get_single() else {
        return;
    };

    let (width, height) = (window.width(), window.height());

    let horizontal_chunk_count = (width / CHUNK_SIZE) as i32 + 1;
    let vertical_chunk_count = (height / CHUNK_SIZE) as i32 + 1;

    let camera_pos: Position = (
        camera_transform.translation.x,
        camera_transform.translation.y,
    )
        .into();

    let start_x = camera_pos.x - horizontal_chunk_count / 2;
    let end_x = camera_pos.x + horizontal_chunk_count / 2;
    let start_y = camera_pos.y - vertical_chunk_count / 2;
    let end_y = camera_pos.y + vertical_chunk_count / 2;

    let mut grid = HashSet::with_capacity((horizontal_chunk_count * vertical_chunk_count) as usize);

    for x in start_x..=end_x {
        for y in start_y..=end_y {
            grid.insert(Position { x, y });
        }
    }

    let chunk_positions: HashSet<_> = tileset.spawned_chunks.keys().cloned().collect();

    // Chunks that are going to be on screen and need to be spawned
    let to_spawn: Vec<_> = grid.difference(&chunk_positions).cloned().collect();

    // Chunks that are no longer on screen and need to be despawned
    let to_despawn: Vec<_> = chunk_positions.difference(&grid).cloned().collect();

    for (position, entity) in tileset.spawned_chunks.clone() {
        if to_despawn.contains(&position) {
            // info!("Despawning Chunk: {position:?}");
            commands.entity(entity).despawn();
            tileset.spawned_chunks.remove(&position);
        }
    }

    if to_spawn.is_empty() {
        return;
    }

    let start = Instant::now();

    let chunk_package: Vec<(Position, Chunk, Image)> = to_spawn
        .par_iter()
        .map(|position| {
            let chunk = generate_chunk(*position, &noisemap.0);
            let texture = chunk_to_image(&chunk);
            (*position, chunk, texture)
        })
        .collect();

    let count = chunk_package.len();

    chunk_package
        .into_iter()
        .for_each(|(position, chunk, texture)| {
            let texture = assets.add(texture);
            spawn_chunk(&mut commands, &mut tileset, chunk, texture, position)
        });

    let end = Instant::now();

    debug!("Spent {:?} spawning {} chunks", end - start, count);
}

/// Generates a `Chunk` from the noisemap for a given position.
fn generate_chunk(position: Position, noisemap: &noisemap::NoiseMap<PerlinNoise>) -> Chunk {
    let chunk_noise = noisemap.generate_chunk(position.x as i64, position.y as i64);

    let mut tiles = [[TileType::Grass; CHUNK_TILE_COUNT]; CHUNK_TILE_COUNT];

    for (row_index, row) in chunk_noise.iter().enumerate() {
        for (tile_index, tile) in row.iter().enumerate() {
            let tile_type = tile.into();

            tiles[row_index][tile_index] = tile_type;
        }
    }

    Chunk(tiles)
}

/// Spawn a chunk with its given texture to the games map.
/// `texture` must correspond to `chunk`.
fn spawn_chunk(
    commands: &mut Commands,
    tileset: &mut ResMut<TileSet>,
    chunk: Chunk,
    texture: Handle<Image>,
    position: Position,
) {
    // Where in the map to to start rendering the chunk, based `position`
    let horizontal_shift = CHUNK_SIZE * (position.x as f32 - 1.0);
    let horizontal_start_pos = (CHUNK_TILE_COUNT as f32 / 2. * TILE_SIZE) + (horizontal_shift);
    let vertical_shift = CHUNK_SIZE * (position.y as f32 - 1.0);
    let vertical_start_pos = (CHUNK_TILE_COUNT as f32 / 2. * TILE_SIZE) + (vertical_shift);

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
            chunk,
            position,
        ))
        .id();

    tileset.spawned_chunks.insert(position, chunk_id);
}

///
fn chunk_to_image(chunk: &Chunk) -> Image {
    let mut dyn_image = DynamicImage::new_rgb16(CHUNK_SIZE as u32, CHUNK_SIZE as u32);

    // Short circuit and fill image completly with one colour if all tiles are the same.
    if let Some(tile_type) = chunk.is_uniform_type() {
        let color: Color = tile_type.into();
        draw_filled_rect_mut(
            &mut dyn_image,
            Rect::at(0, 0).of_size(CHUNK_SIZE as u32, CHUNK_SIZE as u32),
            Rgba(color.as_rgba_u8()),
        );
        return Image::from_dynamic(dyn_image, true, RenderAssetUsages::RENDER_WORLD);
    }

    for (row_index, row) in chunk.0.iter().enumerate() {
        for (tile_index, tile_type) in row.iter().enumerate() {
            let color: Color = tile_type.into();

            draw_filled_rect_mut(
                &mut dyn_image,
                Rect::at(
                    tile_index as i32 * TILE_SIZE as i32,
                    row_index as i32 * TILE_SIZE as i32,
                )
                .of_size(TILE_SIZE as u32, TILE_SIZE as u32),
                Rgba(color.as_rgba_u8()),
            )
        }
    }

    Image::from_dynamic(dyn_image, false, RenderAssetUsages::RENDER_WORLD)
}
