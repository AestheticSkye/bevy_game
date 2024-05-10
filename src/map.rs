mod chunk;
pub mod config;
pub mod position;
mod tile_type;

use std::time::Instant;

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::{HashMap, HashSet};
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{self, NoiseMapGenerator, NoiseMapGeneratorBase, Seed, Step};
use worldgen::world::Size;

use self::chunk::Chunk;
use self::config::MapConfig;
use self::position::Position;
use self::tile_type::TileType;
use crate::get_single;
use crate::player::{sprite_movement, Player};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkReloadEvent>()
            .init_resource::<TileSet>()
            .init_resource::<MapConfig>()
            .init_resource::<NoiseMap>()
            .init_resource::<UnspawnedChunks>()
            .add_systems(Startup, init_noise_map)
            .add_systems(
                Update,
                (chunk_unload, calculate_chunks, spawn_chunks)
                    .chain()
                    .after(sprite_movement),
            );
    }
}

#[derive(Event)]
pub struct ChunkReloadEvent;

#[derive(Resource, Default, Deref, DerefMut)]
struct NoiseMap(noisemap::NoiseMap<PerlinNoise>);

#[derive(Resource, Default, Deref, DerefMut)]
struct TileSet(HashMap<Position, Entity>);

/// A list of unspawned [Chunks](Chunk) that have been generated by
/// [calculate_chunks] that will be then generated and spawned by [spawn_chunks]
#[derive(Resource, Default, Deref, DerefMut)]
struct UnspawnedChunks(Vec<Position>);

fn init_noise_map(mut noisemap: ResMut<NoiseMap>, config: Res<MapConfig>) {
    let noise = PerlinNoise::new();

    noisemap.0 = noisemap::NoiseMap::new(noise)
        .set(Seed::of(rand::random::<i64>())) // Todo: Convert this into a proper seed system
        .set(Size::of(config.chunk_tile_count as i64, config.chunk_tile_count as i64))
        .set(Step::of(0.01, 0.01));
}

/// If a [ChunkReloadEvent] is created, all chunks get unloaded & despawned to then be reloaded.
fn chunk_unload(
    mut commands: Commands,
    mut ev_chunk_reload: EventReader<ChunkReloadEvent>,
    mut noisemap: ResMut<NoiseMap>,
    mut tileset: ResMut<TileSet>,
    config: Res<MapConfig>,
) {
    if !ev_chunk_reload.is_empty() {
        debug!("Unloading all chunks");
        for (_, chunk) in tileset.iter() {
            commands.entity(*chunk).despawn();
        }
        tileset.clear();
        ev_chunk_reload.clear();
        noisemap.0 = noisemap.set(Size::of(
            config.chunk_tile_count as i64,
            config.chunk_tile_count as i64,
        ))
    }
}

/// System to spawn and despawn the games chunks depending on the [Camera] transform.
fn calculate_chunks(
    mut commands: Commands,
    mut tileset: ResMut<TileSet>,
    mut to_spawn: ResMut<UnspawnedChunks>,
    config: Res<MapConfig>,
    camera_transform: Query<&Transform, (With<Camera>, Without<Player>)>,
    camera_projection: Query<&OrthographicProjection, With<Camera>>,
) {
    let camera_transform = get_single!(camera_transform);

    let camera_projection = get_single!(camera_projection);

    let (width, height) = (
        camera_projection.area.width(),
        camera_projection.area.height(),
    );

    // Amount of chunks needed to fill the screen vertically and horizontally.
    let horizontal_chunk_count = (width / config.chunk_size()) as i32 + 1;
    let vertical_chunk_count = (height / config.chunk_size()) as i32 + 1;

    // The chunk position of where the centre of the camera is.
    let camera_pos = Position::from_xy(
        (
            camera_transform.translation.x,
            camera_transform.translation.y,
        ),
        &config,
    );

    let start_x = camera_pos.x - horizontal_chunk_count / 2;
    let end_x = camera_pos.x + horizontal_chunk_count / 2;
    let start_y = camera_pos.y - vertical_chunk_count / 2;
    let end_y = camera_pos.y + vertical_chunk_count / 2;

    let mut grid = HashSet::with_capacity((horizontal_chunk_count * vertical_chunk_count) as usize);

    // Todo: fix the math above so the added positions isnt needed.
    for x in start_x - 1..=end_x + 2 {
        for y in start_y - 1..=end_y + 2 {
            grid.insert(Position { x, y });
        }
    }

    let chunk_positions: HashSet<_> = tileset.keys().cloned().collect();

    // Chunks that are going to be on screen and need to be spawned.
    to_spawn.0 = grid
        .difference(&chunk_positions)
        .cloned()
        .collect::<Vec<Position>>();

    // Chunks that are no longer on screen and need to be despawned.
    let to_despawn: Vec<_> = chunk_positions.difference(&grid).cloned().collect();

    for (position, entity) in tileset.clone() {
        if to_despawn.contains(&position) {
            commands.entity(entity).despawn();
            tileset.remove(&position);
        }
    }
}

/// Takes the chunk positions that were calculated in [calculate_chunks()] and generates and spawns them.
fn spawn_chunks(
    mut commands: Commands,
    mut to_spawn: ResMut<UnspawnedChunks>,
    mut tileset: ResMut<TileSet>,
    mut assets: ResMut<Assets<Image>>,
    config: Res<MapConfig>,
    noisemap: Res<NoiseMap>,
) {
    if to_spawn.is_empty() {
        return;
    }

    let start = Instant::now();

    let chunk_package: Vec<(Position, Chunk, Image)> = to_spawn
        .par_iter()
        .map(|position| {
            let chunk = generate_chunk(*position, &noisemap, &config);
            let texture = chunk_to_image(&chunk, &config);
            (*position, chunk, texture)
        })
        .collect();

    let count = chunk_package.len();

    chunk_package
        .into_iter()
        .for_each(|(position, chunk, texture)| {
            let texture = assets.add(texture);
            render_chunk(
                &mut commands,
                &mut tileset,
                &config,
                chunk,
                texture,
                position,
            )
        });

    to_spawn.clear();

    let end = Instant::now();

    debug!(
        "Spent {:?} spawning {} chunks, for a total of {} loaded",
        end - start,
        count,
        tileset.keys().len()
    );
}

/// Generates a [Chunk] from the noisemap for a given position.
fn generate_chunk(
    position: Position,
    noisemap: &noisemap::NoiseMap<PerlinNoise>,
    config: &MapConfig,
) -> Chunk {
    let chunk_noise = noisemap.generate_chunk(position.x as i64, position.y as i64);

    let mut tiles = vec![vec![TileType::Grass; config.chunk_tile_count]; config.chunk_tile_count];

    for (row_index, row) in chunk_noise.iter().enumerate() {
        for (tile_index, tile) in row.iter().enumerate() {
            let tile_type = tile.into();

            tiles[row_index][tile_index] = tile_type;
        }
    }

    Chunk(tiles)
}

/// Spawn & render a chunk with its given texture to the games map.
/// `texture` must correspond to `chunk`.
fn render_chunk(
    commands: &mut Commands,
    tileset: &mut ResMut<TileSet>,
    config: &MapConfig,
    chunk: Chunk,
    texture: Handle<Image>,
    position: Position,
) {
    // Where in the map to to start rendering the chunk, based `position`
    let horizontal_shift = config.chunk_size() * (position.x as f32 - 1.0);
    let horizontal_start_pos =
        (config.chunk_tile_count as f32 / 2. * config.tile_size) + (horizontal_shift);
    let vertical_shift = config.chunk_size() * (position.y as f32 - 1.0);
    let vertical_start_pos =
        (config.chunk_tile_count as f32 / 2. * config.tile_size) + (vertical_shift);

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

    tileset.insert(position, chunk_id);
}

/// Convert a [Chunk] and its data into a bevy [bevy_render::texture::image::Image] to be used for creating textures.
fn chunk_to_image(chunk: &Chunk, config: &MapConfig) -> Image {
    let mut dyn_image =
        DynamicImage::new_rgb8(config.chunk_size() as u32, config.chunk_size() as u32);

    // Short circuit and fill image completely with one colour if all tiles are the same.
    if let Some(tile_type) = chunk.is_uniform_type() {
        let color: Color = tile_type.into();
        draw_filled_rect_mut(
            &mut dyn_image,
            Rect::at(0, 0).of_size(config.chunk_size() as u32, config.chunk_size() as u32),
            Rgba(color.as_rgba_u8()),
        );
        return Image::from_dynamic(dyn_image, true, RenderAssetUsages::RENDER_WORLD);
    }

    for (row_index, row) in chunk.0.iter().rev().enumerate() {
        for (tile_index, tile_type) in row.iter().enumerate() {
            let color: Color = tile_type.into();

            draw_filled_rect_mut(
                &mut dyn_image,
                Rect::at(
                    tile_index as i32 * config.tile_size as i32,
                    row_index as i32 * config.tile_size as i32,
                )
                .of_size(config.tile_size as u32, config.tile_size as u32),
                Rgba(color.as_rgba_u8()),
            )
        }
    }

    Image::from_dynamic(dyn_image, true, RenderAssetUsages::RENDER_WORLD)
}
