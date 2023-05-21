#![allow(non_snake_case)]
use dioxus::prelude::*;
use super::source_plate::SourcePlate;
use super::destination_plate::DestinationPlate;

static STYLE: &'static str = include_str!("plate_container.css");

#[inline_props]
pub fn PlateContainer(cx: Scope, source_dims: (u8,u8), destination_dims: (u8,u8)) -> Element {
    cx.render(rsx! {
        style { STYLE }
        div {
            class: "plate_container",
            SourcePlate {width: source_dims.0,
                         height: source_dims.1},
            DestinationPlate {width: destination_dims.0,
                              height: destination_dims.1}
        }
    })
}
