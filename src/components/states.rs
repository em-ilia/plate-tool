use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yewdux::{prelude::*, storage};

use super::transfer_menu::RegionDisplay;
use crate::data::plate::*;
use crate::data::plate_instances::PlateInstance;
use crate::data::transfer::Transfer;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "session")]
pub struct CurrentTransfer {
    pub transfer: Transfer,
}

#[derive(Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MainState {
    pub source_plates: Vec<PlateInstance>,
    pub destination_plates: Vec<PlateInstance>,
    pub transfers: Vec<Transfer>,
    pub selected_source_plate: Uuid,
    pub selected_dest_plate: Uuid,
    pub selected_transfer: Uuid,
}

impl Store for MainState {
    fn new() -> Self {
        init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));

        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
        /*
        Self {
            source_plates: Vec::new(),
            destination_plates: Vec::new(),
            transfers: Vec::new(),
        }
        */
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl MainState {
    pub fn purge_transfers(&mut self) {
        // Removes any transfers for which the associated plates are gone
        self.transfers = self
            .transfers
            .iter()
            .filter(|tr| {
                self.source_plates
                    .iter()
                    .any(|spi| spi.get_uuid() == tr.source_id)
                    && self
                        .destination_plates
                        .iter()
                        .any(|dpi| dpi.get_uuid() == tr.dest_id)
            })
            .map(|tr| tr.clone())
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
    pub fn del_plate(&mut self, id: Uuid) {
        if let Some(index) = self
            .source_plates
            .iter()
            .position(|spi| spi.get_uuid() == id)
        {
            self.source_plates.swap_remove(index);
        }
        if let Some(index) = self
            .destination_plates
            .iter()
            .position(|dpi| dpi.get_uuid() == id)
        {
            self.destination_plates.swap_remove(index);
        }
    }
    pub fn rename_plate(&mut self, id: Uuid, new_name: &str) {
        if let Some(index) = self
            .source_plates
            .iter()
            .position(|spi| spi.get_uuid() == id)
        {
            self.source_plates[index].change_name(new_name.to_string());
        }
        if let Some(index) = self
            .destination_plates
            .iter()
            .position(|dpi| dpi.get_uuid() == id)
        {
            self.destination_plates[index].change_name(new_name.to_string());
        }
    }
}
