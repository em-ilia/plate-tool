#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn Tree(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "tree",
        }
    })
}
