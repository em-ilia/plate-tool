#![allow(non_snake_case)]

use std::collections::HashMap;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::states::{CurrentTransfer, MainState};
use crate::data::plate_instances::PlateInstance;
use crate::data::transfer::Transfer;
use crate::data::transfer_region::Region;

// Color Palette for the Source Plates, can be changed here
use crate::components::plates::util::Palettes;
const PALETTE: super::util::ColorPalette = Palettes::RAINBOW;

use super::super::transfer_menu::{num_to_letters, RegionDisplay};

#[derive(PartialEq, Properties)]
pub struct SourcePlateProps {
    pub source_plate: PlateInstance,
    pub destination_plate: PlateInstance,
    pub cell_height: f64,
}

#[function_component]
pub fn SourcePlate(props: &SourcePlateProps) -> Html {
    let (main_state, _) = use_store::<MainState>();
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();
    let m_start_handle: UseStateHandle<Option<(u8, u8)>> = use_state_eq(|| None);
    let m_end_handle: UseStateHandle<Option<(u8, u8)>> = use_state_eq(|| None);
    let m_stat_handle: UseStateHandle<bool> = use_state_eq(|| false);

    if !(*m_stat_handle) {
        let (pt1, pt2) = match ct_state.transfer.transfer_region.source_region {
            Region::Point((x, y)) => ((x, y), (x, y)),
            Region::Rect(c1, c2) => (c1, c2),
            Region::Custom(_) => ((0,0), (0,0)),
        };
        m_start_handle.set(Some(pt1));
        m_end_handle.set(Some(pt2));
    }

    let transfer_map = {
        let ts = main_state
            .transfers
            .iter()
            .filter(|t| t.source_id == props.source_plate.get_uuid());
        let mut tooltip_map: HashMap<(u8, u8), Vec<&Transfer>> = HashMap::new();
        for t in ts {
            let sws = t.transfer_region.get_source_wells();
            for sw in sws {
                if let Some(val) = tooltip_map.get_mut(&sw) {
                    val.push(t);
                } else {
                    tooltip_map.insert(sw, vec![t]);
                }
            }
        }
        tooltip_map
    };

    let source_wells = ct_state.transfer.transfer_region.get_source_wells();

    let ordered_ids: Vec<uuid::Uuid> = {
        let mut ids: Vec<uuid::Uuid> = main_state.transfers.clone().iter()
        .map(|x| x.id)
        .collect();
        ids.sort_unstable();
        ids
    };

    let mouse_callback = {
        let m_start_handle = m_start_handle.clone();
        let m_end_handle = m_end_handle.clone();
        let m_stat_handle = m_stat_handle.clone();

        Callback::from(move |(i, j, t)| match t {
            MouseEventType::Mousedown => {
                m_start_handle.set(Some((i, j)));
                m_end_handle.set(Some((i, j)));
                m_stat_handle.set(true);
            }
            MouseEventType::Mouseenter => {
                if *m_stat_handle {
                    m_end_handle.set(Some((i, j)));
                }
            }
        })
    };

    let mouseup_callback = {
        let m_start_handle = m_start_handle.clone();
        let m_end_handle = m_end_handle.clone();

        Callback::from(move |_: MouseEvent| {
            m_stat_handle.set(false);
            if let Some(ul) = *m_start_handle {
                if let Some(br) = *m_end_handle {
                    if let Ok(rd) = RegionDisplay::try_from((ul.0, ul.1, br.0, br.1)) {
                        ct_dispatch.reduce_mut(|state| {
                            state.transfer.transfer_region.source_region = Region::from(&rd);
                        });
                    }
                }
            }
        })
    };

    let mouseleave_callback = Callback::clone(&mouseup_callback);

    let column_header = {
        let headers = (1..=props.source_plate.plate.size().1)
            .map(|j| {
                html! {<th>
                {format!("{:0>2}", j)}
                </th>}
            })
            .collect::<Html>();
        html! {<tr><th />{ headers }</tr>}
    };
    let rows = (1..=props.source_plate.plate.size().0)
        .map(|i| {
            let row_header = html! {<th>{num_to_letters(i)}</th>};
            let row = (1..=props.source_plate.plate.size().1)
                .map(|j| {
                    html! {
                        <SourcePlateCell i={i} j={j}
                        selected={in_rect(*m_start_handle.clone(), *m_end_handle.clone(), (i,j))}
                        mouse={mouse_callback.clone()}
                        in_transfer={source_wells.contains(&(i,j))}
                        color={transfer_map.get(&(i,j))
                            .and_then(|t| t.last())
                            .map(|t| PALETTE.get_ordered(t.get_uuid(), &ordered_ids))
                        }
                        cell_height={props.cell_height}
                        title={transfer_map.get(&(i,j)).map(|transfers| format!("Used by: {}", transfers.iter().map(|t| t.name.clone())
                                    .collect::<Vec<_>>().join(", ")))}
                        />
                    }
                })
                .collect::<Html>();
            html! {
                <tr>
                    { row_header }{ row }
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <div class={classes!{"source_plate",
        "W".to_owned()+&props.source_plate.plate.plate_format.to_string()}}>
            <table
            onmouseup={move |e| {
                mouseup_callback.emit(e);
            }}
            onmouseleave={move |e| {
                mouseleave_callback.emit(e);
            }}>
                { column_header }
                { rows }
            </table>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct SourcePlateCellProps {
    i: u8,
    j: u8,
    selected: bool,
    mouse: Callback<(u8, u8, MouseEventType)>,
    in_transfer: Option<bool>,
    color: Option<[f64; 3]>,
    cell_height: f64,
    title: Option<String>,
}
#[derive(Debug)]
pub enum MouseEventType {
    Mousedown,
    Mouseenter,
}

#[function_component]
fn SourcePlateCell(props: &SourcePlateCellProps) -> Html {
    let selected_class = match props.selected {
        true => Some("current_select"),
        false => None,
    };
    let in_transfer_class = match props.in_transfer {
        Some(true) => Some("in_transfer"),
        _ => None,
    };
    let color = props.color.unwrap_or([255.0, 255.0, 255.0]);
    let mouse = Callback::clone(&props.mouse);
    let mouse2 = Callback::clone(&props.mouse);
    let (i, j) = (props.i, props.j);

    html! {
        <td class={classes!("plate_cell", selected_class, in_transfer_class)}
            style={format!("height: {}px;", props.cell_height)}
            id={format!("color={:?}", props.color)}
            onmousedown={move |_| {
                mouse.emit((i,j, MouseEventType::Mousedown))
            }}
            onmouseenter={move |_| {
                mouse2.emit((i,j, MouseEventType::Mouseenter))
            }}>
            <div class="plate_cell_inner"
            style={format!("background: rgba({},{},{},1);", color[0], color[1], color[2])}
            title={if let Some(text) = &props.title {
                text.clone()
            } else {"".to_string()}}/>
        </td>
    }
}

pub fn in_rect(corner1: Option<(u8, u8)>, corner2: Option<(u8, u8)>, pt: (u8, u8)) -> bool {
    if let (Some(c1), Some(c2)) = (corner1, corner2) {
        pt.0 <= u8::max(c1.0, c2.0)
            && pt.0 >= u8::min(c1.0, c2.0)
            && pt.1 <= u8::max(c1.1, c2.1)
            && pt.1 >= u8::min(c1.1, c2.1)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::in_rect;

    // in_rect tests
    #[test]
    #[wasm_bindgen_test]
    fn test_in_rect1() {
        // Test in center of rect
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt = (5, 5);
        assert!(in_rect(Some(c1), Some(c2), pt));
        // Order of the corners should not matter:
        assert!(in_rect(Some(c2), Some(c1), pt));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_in_rect2() {
        // Test on top/bottom edges of rect
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (1, 5);
        let pt2 = (10, 5);
        assert!(in_rect(Some(c1), Some(c2), pt1));
        assert!(in_rect(Some(c1), Some(c2), pt2));
        // Order of the corners should not matter:
        assert!(in_rect(Some(c2), Some(c1), pt1));
        assert!(in_rect(Some(c2), Some(c1), pt2));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_in_rect3() {
        // Test on left/right edges of rect
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (5, 1);
        let pt2 = (5, 10);
        assert!(in_rect(Some(c1), Some(c2), pt1));
        assert!(in_rect(Some(c1), Some(c2), pt2));
        // Order of the corners should not matter:
        assert!(in_rect(Some(c2), Some(c1), pt1));
        assert!(in_rect(Some(c2), Some(c1), pt2));
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_in_rect4() {
        // Test cases that should fail
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (0, 0);
        let pt2 = (15, 15);
        assert!(!in_rect(Some(c1), Some(c2), pt1));
        assert!(!in_rect(Some(c1), Some(c2), pt2));
    }
}
