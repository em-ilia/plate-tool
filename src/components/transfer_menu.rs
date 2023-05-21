#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn TransferMenu(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "transfer_menu",
        }
    })
}
