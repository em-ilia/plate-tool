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

pub enum PlateType {
    Source,
    Destination,
}

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

/*
#[cfg(test)]
mod tests {
    use super::{Plate, PlateFormat, PlateType};
}
*/
