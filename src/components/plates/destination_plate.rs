#![allow(non_snake_case)]
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DestinationPlateProps {
    pub width: u8,
    pub height: u8,
}

#[function_component]
pub fn DestinationPlate(props: &DestinationPlateProps) -> Html {
    let rows = (1..=props.height).map(|i| {
        let row = (1..=props.width).map(|j| {
            html! {
                <DestPlateCell i={i} j={j} />
            }
        }).collect::<Html>();
        html! {
            <tr>
                { row }
            </tr>
        }
    }).collect::<Html>();

    html! {
        <div class="dest_plate">
            <table>
                { rows }
            </table>
        </div>
    }
    /*
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
    */
}

#[derive(Properties, PartialEq)]
pub struct DestPlateCellProps {
    pub i: u8,
    pub j: u8,
    pub color: Option<String>
}

#[function_component]
fn DestPlateCell(props: &DestPlateCellProps) -> Html {
    let color_string = match &props.color {
        Some(c) => c.clone(),
        None => "None".to_string(),
    };

    html! {
        <td class="plate_cell">
            <div class="plate_cell_inner" />
        </td>
    }
    /*
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
    */
}
