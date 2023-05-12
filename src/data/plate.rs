pub struct Plate {
    pub plate_type: PlateType,
    pub plate_format: PlateFormat,
    well_groups: Vec<u8>
}

impl Plate {
    pub fn new(plate_type: PlateType, plate_format: PlateFormat) -> Self {
        let (l,w) = plate_format.size();
        Plate {
            plate_type,
            plate_format,
            well_groups: Vec::with_capacity((l*w) as usize)
        }
    }

    pub fn size(&self) -> (u8,u8) {
        self.plate_format.size()
    }

    pub fn get_well_group(&self, i: u8, j: u8) -> u8 {
        self.well_groups[ ((i-1)*self.size().1 + (j-1)) as usize ]
    }
}

pub enum PlateType {
    Source,
    Destination
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
    pub fn size(&self) -> (u8,u8) {
        match self {
            PlateFormat::W6 => (2,3),
            PlateFormat::W12 => (3,4),
            PlateFormat::W24 => (4,6),
            PlateFormat::W48 => (6,8),
            PlateFormat::W96 => (8,12),
            PlateFormat::W384 => (16,24),
            PlateFormat::W1536 => (32,48),
            PlateFormat::W3456 => (48,72),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Plate, PlateFormat, PlateType};

    #[test]
    fn test_get_well_group() {
        let plate = Plate { // Plate where we know every plate group number
            plate_type: PlateType::Source,
            plate_format: PlateFormat::W12,
            well_groups: vec![1,2,3,4,5,6,7,8,9,10,11,12]
        };

        assert_eq!(plate.get_well_group(1, 3), 3);
        assert_eq!(plate.get_well_group(2, 3), 7);
        assert_eq!(plate.get_well_group(3, 1), 9);
    }
}
