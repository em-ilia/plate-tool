#![allow(non_snake_case)]
mod components;

use dioxus::prelude::*;
use components::source_plate::SourcePlate;

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
