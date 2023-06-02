use super::plate::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct PlateInstance {
    pub plate: Plate,
    id: Uuid,
    pub name: String,
}

impl PlateInstance {
    pub fn new(sort: PlateType, format: PlateFormat, name: String) -> Self {
        PlateInstance {
            plate: Plate {
                plate_type: sort,
                plate_format: format,
            },
            id: Uuid::new_v4(),
            name,
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.id
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

impl From<Plate> for PlateInstance {
    fn from(value: Plate) -> Self {
        PlateInstance {
            plate: value,
            id: Uuid::new_v4(),
            name: "New Plate".to_string(),
        }
    }
}
