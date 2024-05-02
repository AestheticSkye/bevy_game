use bevy::render::color::Color;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum TileType {
    Water,
    Grass,
    Sand,
}

impl From<TileType> for Color {
    fn from(val: TileType) -> Self {
        match val {
            TileType::Water => Color::BLUE,
            TileType::Grass => Color::GREEN,
            TileType::Sand => Color::YELLOW,
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
        } else {
            TileType::Water
        }
    }
}

impl From<&f64> for TileType {
    fn from(val: &f64) -> Self { (*val).into() }
}
