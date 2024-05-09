use bevy::render::color::Color;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum TileType {
    Water,
    DeepWater,
    HighGrass,
    Grass,
    Sand,
}

impl From<TileType> for Color {
    fn from(val: TileType) -> Self {
        match val {
            TileType::Water => Color::hex("2600FE"),
            TileType::DeepWater => Color::hex("2200E6"),
            TileType::Grass => Color::hex("54BE44"),
            TileType::HighGrass => Color::hex("4AAD40"),
            TileType::Sand => Color::hex("FDF1D4"),
        }
        .unwrap()
    }
}

impl From<&TileType> for Color {
    fn from(val: &TileType) -> Self { (*val).into() }
}

impl From<f64> for TileType {
    fn from(val: f64) -> Self {
        if val > 0.4 {
            TileType::HighGrass
        } else if val > 0.1 {
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
