#![allow(non_snake_case)]
mod components;
mod data;

use components::main_window::MainWindow;
use dioxus::prelude::*;

#[cfg(debug_assertions)]
use data::*;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        MainWindow {}
    })
}

pub fn plate_test() {
    let source = plate::Plate::new(plate::PlateType::Source, plate::PlateFormat::W96);
    let destination = plate::Plate::new(plate::PlateType::Destination, plate::PlateFormat::W384);

    let transfer = transfer_region::TransferRegion {
        source_plate: &source,
        source_region: transfer_region::Region::Rect((1, 1), (2, 2)),
        dest_plate: &destination,
        dest_region: transfer_region::Region::Rect((2,2),(11,11)),
        interleave_source: None,
        interleave_dest: Some((3,3)),
    };
    println!("{}", transfer);
    let sws = transfer.get_source_wells();
    let m = transfer.calculate_map();
    for w in sws {
        println!("{:?} -> {:?}", w,m(w));
    }
}
