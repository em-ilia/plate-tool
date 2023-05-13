#![allow(non_snake_case)]
mod components;
mod data;

use components::source_plate::SourcePlate;
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use data::*;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Shrimp"
            SourcePlate {
                width: 24,
                height: 18,
            }
        }
    })
}

pub fn plate_test() {
    let source = plate::Plate::new(plate::PlateType::Source, plate::PlateFormat::W96);
    let destination = plate::Plate::new(plate::PlateType::Destination, plate::PlateFormat::W96);

    let transfer = transfer_region::TransferRegion {
        source_plate: &source,
        source_region: transfer_region::Region::Rect((1,1), (6,6)),
        dest_plate: &destination,
        dest_region: transfer_region::Region::Point((2,2)),
        interleave_source: None,
        interleave_dest: None
    };
    println!("{}", transfer);
    println!("{:?}", transfer.get_source_wells());
    println!("{:?}", transfer.get_destination_wells());
}
