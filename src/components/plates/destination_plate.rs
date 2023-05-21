#![allow(non_snake_case)]
use dioxus::prelude::*;

#[inline_props]
pub fn DestinationPlate(cx: Scope, width: u8, height: u8) -> Element {
    cx.render(rsx! {
        div {
            class: "dest_plate",
            table {
                for i in 1..=cx.props.height {
                    tr {
                        draggable: "false",
                        for j in 1..=cx.props.width {
                            DestPlateCell {i: i, j: j}
                        }
                    }
                },
            }
        }
    })
}

#[inline_props]
fn DestPlateCell(cx: Scope<PlateCellProps>, i: u8, j: u8, color: Option<String>) -> Element {
    let color_string = match color {
        Some(c) => c.clone(),
        None => "None".to_string(),
    };

    cx.render(rsx! {
        td {
            class: "plate_cell",
            draggable: "false",
            style: "background: {color_string}",
            div {
                class: "plate_cell_inner"
            }
        }
    })
}
