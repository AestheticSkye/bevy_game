use bevy::prelude::*;
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{self, NoiseMapGenerator, NoiseMapGeneratorBase, Seed, Step};
use worldgen::world::Size;

const TILE_SIZE: f32 = 50.0;
const CHUNK_SIZE: i32 = 15;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(generate_noise_map())
            .add_systems(Startup, spawn_tiles);
    }
}

#[derive(Resource)]
struct NoiseMap(noisemap::NoiseMap<PerlinNoise>);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

fn generate_noise_map() -> NoiseMap {
    let noise = PerlinNoise::new();

    NoiseMap(
        noisemap::NoiseMap::new(noise)
        .set(Seed::of(rand::random::<i64>())) // Todo: Convert this into a propper seet system
        .set(Size::of(CHUNK_SIZE as i64, CHUNK_SIZE as i64))
        .set(Step::of(0.08, 0.08)),
    )
}

enum TileType {
    Water,
    Grass,
    Sand,
}

fn generate_chunk(
    commands: &mut Commands,
    position: Position,
    noisemap: &noisemap::NoiseMap<PerlinNoise>,
) {
    let chunk_noise = noisemap.generate_chunk(position.x as i64, position.y as i64);

    // Where in the map to to start rendering the chunk, based `position`
    let horizontal_shift = CHUNK_SIZE as f32 * TILE_SIZE * (position.x as f32 - 1.0);
    let horizontal_start_pos = (CHUNK_SIZE as f32 / 2. * TILE_SIZE) + (horizontal_shift);

    let vertical_shift = CHUNK_SIZE as f32 * TILE_SIZE * (position.y as f32 - 1.0);
    let vertical_start_pos = (CHUNK_SIZE as f32 / 2. * TILE_SIZE) + (vertical_shift);

    for (row_index, row) in chunk_noise.iter().enumerate() {
        for (tile_index, tile) in row.iter().enumerate() {
            let tile_type = to_tile_type(*tile);

            let color = match tile_type {
                TileType::Water => Color::BLUE,
                TileType::Grass => Color::GREEN,
                TileType::Sand => Color::YELLOW,
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    transform: Transform {
                        translation: Vec3::new(
                            horizontal_start_pos + TILE_SIZE * tile_index as f32,
                            vertical_start_pos + TILE_SIZE * row_index as f32,
                            -1.0,
                        ),
                        rotation:    Quat::default(),
                        scale:       Vec3::new(TILE_SIZE, TILE_SIZE, 0.0),
                    },
                    ..default()
                },
                Tile(tile_type),
            ));
        }
    }
}

fn to_tile_type(input: f64) -> TileType {
    if input > -0.3 {
        TileType::Water
    } else if input > -0.4 {
        TileType::Sand
    } else {
        TileType::Grass
    }
}

#[derive(Component)]
struct Tile(TileType);

fn spawn_tiles(mut commands: Commands, noisemap: Res<NoiseMap>) {
    for i in -3..=3 {
        for j in -3..=3 {
            generate_chunk(&mut commands, Position { x: i, y: j }, &noisemap.0);
        }
    }
}
