use bevy::render::color::Color;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum TileType {
    Water,
    DeepWater,
    Grass,
    Sand,
}

impl From<TileType> for Color {
    fn from(val: TileType) -> Self {
        match val {
            TileType::Water => Color::BLUE,
            TileType::Grass => Color::rgb(0.0705, 0.5215, 0.0627),
            TileType::Sand => Color::rgb(0.9843, 0.8823, 0.6627),
            TileType::DeepWater => Color::rgb(0.011, 0.0, 0.8),
        }
    }
}

impl From<&TileType> for Color {
    fn from(val: &TileType) -> Self { (*val).into() }
}

impl From<f64> for TileType {
    fn from(val: f64) -> Self {
        if val > 0.1 {
            TileType::Grass
        } else if val > 0.0 {
            TileType::Sand
        } else if val > -0.5 {
            TileType::Water
        } else {
            TileType::DeepWater
        }
    }
}

impl From<&f64> for TileType {
    fn from(val: &f64) -> Self { (*val).into() }
}
