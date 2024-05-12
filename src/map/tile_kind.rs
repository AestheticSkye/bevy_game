use bevy::render::color::Color;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum TileKind {
    Water,
    DeepWater,
    HighGrass,
    Grass,
    Sand,
}

#[allow(clippy::fallible_impl_from)]
impl From<TileKind> for Color {
    fn from(val: TileKind) -> Self {
        match val {
            TileKind::Water => Self::hex("2600FE"),
            TileKind::DeepWater => Self::hex("2200E6"),
            TileKind::Grass => Self::hex("54BE44"),
            TileKind::HighGrass => Self::hex("4AAD40"),
            TileKind::Sand => Self::hex("FDF1D4"),
        }
        .unwrap()
    }
}

impl From<&TileKind> for Color {
    fn from(val: &TileKind) -> Self { (*val).into() }
}

impl From<f64> for TileKind {
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

impl From<&f64> for TileKind {
    fn from(val: &f64) -> Self { (*val).into() }
}
