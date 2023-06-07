use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Plate {
    pub plate_type: PlateType,
    pub plate_format: PlateFormat,
}

impl Plate {
    pub fn new(plate_type: PlateType, plate_format: PlateFormat) -> Self {
        Plate {
            plate_type,
            plate_format,
        }
    }

    pub fn size(&self) -> (u8, u8) {
        self.plate_format.size()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PlateType {
    Source,
    Destination,
}
impl Default for PlateType {
    fn default() -> Self {
        Self::Source
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PlateFormat {
    W6,
    W12,
    W24,
    W48,
    W96,
    W384,
    W1536,
    W3456,
}
impl Default for PlateFormat {
    fn default() -> Self {
        Self::W96
    }
}
impl std::fmt::Display for PlateFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlateFormat::W6 => write!(f, "6"),
            PlateFormat::W12 => write!(f, "12"),
            PlateFormat::W24 => write!(f, "24"),
            PlateFormat::W48 => write!(f, "48"),
            PlateFormat::W96 => write!(f, "96"),
            PlateFormat::W384 => write!(f, "384"),
            PlateFormat::W1536 => write!(f, "1536"),
            PlateFormat::W3456 => write!(f, "3456"),
        }
    }
}

impl PlateFormat {
    pub fn size(&self) -> (u8, u8) {
        match self {
            PlateFormat::W6 => (2, 3),
            PlateFormat::W12 => (3, 4),
            PlateFormat::W24 => (4, 6),
            PlateFormat::W48 => (6, 8),
            PlateFormat::W96 => (8, 12),
            PlateFormat::W384 => (16, 24),
            PlateFormat::W1536 => (32, 48),
            PlateFormat::W3456 => (48, 72),
        }
    }
}
