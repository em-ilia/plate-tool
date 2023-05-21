#![allow(non_snake_case)]
use dioxus::prelude::*;
use super::plates::plate_container::PlateContainer;
use super::tree::Tree;
use super::transfer_menu::TransferMenu;

static STYLE: &'static str = include_str!("global.css");

pub fn MainWindow(cx: Scope) -> Element {
    cx.render(rsx! {
        style { STYLE },
        div {
            class: "main_container",
            Tree {},
            TransferMenu {},
            PlateContainer {
                source_dims: (24,16),
                destination_dims: (24,16)
            }
        }
    })
}
