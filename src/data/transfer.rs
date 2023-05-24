use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;
use super::transfer_region::*;
use super::plate_instances::*;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Transfer {
    pub source_id: Uuid,
    pub dest_id: Uuid,
    pub transfer_region: TransferRegion,
}

impl Transfer {
    fn new(source: PlateInstance, dest: PlateInstance, tr: TransferRegion) -> Self {
        Self {
            source_id: source.get_uuid(),
            dest_id: dest.get_uuid(),
            transfer_region: tr
        }
    }
}
