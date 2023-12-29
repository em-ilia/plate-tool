use super::plate_instances::*;
use super::transfer_region::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Transfer {
    pub source_id: Uuid,
    pub dest_id: Uuid,
    pub name: String,
    #[serde(rename = "id_v7")]
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    pub transfer_region: TransferRegion,
    #[serde(default = "default_volume")]
    pub volume: f32,
}

impl Default for Transfer {
    fn default() -> Self {
        Transfer {
            source_id: Default::default(),
            dest_id: Default::default(),
            name: "New Transfer".to_string(),
            id: Default::default(),
            transfer_region: Default::default(),
            volume: 2.5f32,
        }
    }
}

fn default_volume() -> f32 {
    2.5f32
}

impl Transfer {
    pub fn new(
        source: PlateInstance,
        dest: PlateInstance,
        tr: TransferRegion,
        name: String,
    ) -> Self {
        Self {
            source_id: source.get_uuid(),
            dest_id: dest.get_uuid(),
            name,
            id: Uuid::now_v7(),
            transfer_region: tr,
            volume: 2.5,
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.id
    }
}
