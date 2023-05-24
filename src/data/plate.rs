use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum PlateType {
    Source,
    Destination,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
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
            PlateFormat::W6    => (3, 2),
            PlateFormat::W12   => (4, 3),
            PlateFormat::W24   => (6, 4),
            PlateFormat::W48   => (8, 6),
            PlateFormat::W96   => (12, 8),
            PlateFormat::W384  => (24, 16),
            PlateFormat::W1536 => (48, 32),
            PlateFormat::W3456 => (72, 48),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::{Plate, PlateFormat, PlateType};
}
*/
