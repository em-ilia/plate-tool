use yewdux::prelude::*;
use super::transfer_menu::RegionDisplay;

#[derive(Debug, Default, Clone, PartialEq, Store)]
pub struct NewTransferState {
    pub source_region: RegionDisplay,
    pub destination_region: RegionDisplay,
    pub interleave_x: u8,
    pub interleave_y: u8,
}
