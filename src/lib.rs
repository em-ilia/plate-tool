#![allow(non_snake_case)]
mod components;
mod data;

use components::source_plate::SourcePlate;
use dioxus::prelude::*;

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
