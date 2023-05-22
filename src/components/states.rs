use yewdux::prelude::*;
use super::transfer_menu::RegionDisplay;
use crate::data::plate_instances::PlateInstance;
use crate::data::transfer::Transfer;
use crate::data::plate::*;

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct NewTransferState {
    pub source_region: RegionDisplay,
    pub destination_region: RegionDisplay,
    pub interleave_x: u8,
    pub interleave_y: u8,
}

#[derive(Default, PartialEq, Clone)]
pub struct MainState {
    pub source_plates: Vec<PlateInstance>,
    pub destination_plates: Vec<PlateInstance>,
    pub transfers: Vec<Transfer>,
}

impl Store for MainState {
    fn new() -> Self {
        Self {
            source_plates: Vec::new(),
            destination_plates: Vec::new(),
            transfers: Vec::new(),
        }
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl MainState {
    pub fn purge_transfers(&mut self) {
        // Removes any transfers for which the associated plates are gone
        self.transfers = self.transfers.iter()
            .filter(|tr| {
                self.source_plates.iter().any(|spi| {
                    spi.get_uuid() == tr.source_id
                }) &&
                self.destination_plates.iter().any(|dpi| {
                    dpi.get_uuid() == tr.dest_id
                })
            })
            .map(|&tr| tr)
            .collect();
    }

    pub fn add_source_plate(&mut self, plate: PlateInstance) {
        assert!(plate.plate.plate_type == PlateType::Source);
        self.source_plates.push(plate);
    }
    pub fn add_dest_plate(&mut self, plate: PlateInstance) {
        assert!(plate.plate.plate_type == PlateType::Destination);
        self.destination_plates.push(plate);
    }
}
