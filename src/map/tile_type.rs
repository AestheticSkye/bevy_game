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

#[allow(clippy::fallible_impl_from)]
impl From<TileType> for Color {
    fn from(val: TileType) -> Self {
        match val {
            TileType::Water => Self::hex("2600FE"),
            TileType::DeepWater => Self::hex("2200E6"),
            TileType::Grass => Self::hex("54BE44"),
            TileType::HighGrass => Self::hex("4AAD40"),
            TileType::Sand => Self::hex("FDF1D4"),
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
            Self::HighGrass
        } else if val > 0.1 {
            Self::Grass
        } else if val > 0.0 {
            Self::Sand
        } else if val > -0.5 {
            Self::Water
        } else {
            Self::DeepWater
        }
    }
}

impl From<&f64> for TileType {
    fn from(val: &f64) -> Self { (*val).into() }
}
