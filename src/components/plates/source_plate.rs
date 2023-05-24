#![allow(non_snake_case)]

use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;

use crate::data::plate_instances::PlateInstance;
use crate::data::transfer_region::{TransferRegion, Region};
use crate::components::states::{NewTransferState, CurrentTransfer};

use super::super::transfer_menu::RegionDisplay;

#[derive(PartialEq, Properties)]
pub struct SourcePlateProps {
    pub source_plate: PlateInstance,
    pub destination_plate: PlateInstance,
}

#[function_component]
pub fn SourcePlate(props: &SourcePlateProps) -> Html {
    let (state, dispatch) = use_store::<NewTransferState>();
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();
    let m_start_handle: UseStateHandle<Option<(u8,u8)>> = use_state_eq(|| None);
    let m_end_handle: UseStateHandle<Option<(u8,u8)>> = use_state_eq(|| None);
    let m_stat_handle: UseStateHandle<bool> = use_state_eq(|| false);
    let m_start = m_start_handle.clone();
    let m_end = m_end_handle.clone();

    if !(*m_stat_handle) {
        let pt1 = (state.source_region.col_start, state.source_region.row_start);
        let pt2 = (state.source_region.col_end, state.source_region.row_end);
        m_start_handle.set(Some(pt1));
        m_end_handle.set(Some(pt2));
    }

    let source_wells = ct_state.transfer.get_source_wells();

    let mouse_callback = {
        let m_start_handle = m_start_handle.clone();
        let m_end_handle = m_end_handle.clone();
        let m_stat_handle = m_stat_handle.clone();

        Callback::from(move |(i,j,t)| {
            match t {
                MouseEventType::MOUSEDOWN => {
                    m_start_handle.set(Some((i,j)));
                    m_end_handle.set(Some((i,j)));
                    m_stat_handle.set(true);
                },
                MouseEventType::MOUSEENTER => {
                    if *m_stat_handle {
                        m_end_handle.set(Some((i,j)));
                    }
                }
            }
        })
    };

    let mouseup_callback = {
        let m_start_handle = m_start_handle.clone();
        let m_end_handle = m_end_handle.clone();
        let m_stat_handle = m_stat_handle.clone();

        Callback::from(move |_: MouseEvent| {
        m_stat_handle.set(false);
        if let Some(ul) = *m_start_handle {
            if let Some(br) = *m_end_handle {
                if let Ok(rd) = RegionDisplay::try_from((ul.0, ul.1, br.0, br.1)) {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.source_region = Region::from(&rd);
                    });
                    dispatch.reduce_mut(|state| {
                        state.source_region = rd;
                    });
                }
            }
        }
        })
    };

    let mouseleave_callback = Callback::clone(&mouseup_callback);

    let rows = (1..=props.source_plate.plate.size().0).map(|i| {
        let row = (1..=props.source_plate.plate.size().1).map(|j| {
            html! {
                <SourcePlateCell i={i} j={j}
                selected={in_rect(*m_start.clone(), *m_end.clone(), (i,j))}
                mouse={mouse_callback.clone()}
                in_transfer={source_wells.contains(&(i,j))}
                />
            }
        }).collect::<Html>();
        html! {
            <tr>
                { row }
            </tr>
        }
    }).collect::<Html>();

    html! {
        <div class="source_plate">
            <table
            onmouseup={move |e| {
                mouseup_callback.emit(e);
            }}
            onmouseleave={move |e| {
                mouseleave_callback.emit(e);
            }}>
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
    mouse: Callback<(u8,u8, MouseEventType)>,
    in_transfer: Option<bool>,
}
#[derive(Debug)]
pub enum MouseEventType {
    MOUSEDOWN,
    MOUSEENTER
}

#[function_component]
fn SourcePlateCell(props: &SourcePlateCellProps) -> Html {
    let selected_class = match props.selected {
        true => Some("current_select"),
        false => None,
    };
    let in_transfer_class = match props.in_transfer {
        Some(true) => Some("in_transfer"),
        _ => None
    };
    let mouse = Callback::clone(&props.mouse);
    let mouse2 = Callback::clone(&props.mouse);
    let (i,j) = (props.i.clone(), props.j.clone());

    html! {
        <td class={classes!("plate_cell", selected_class, in_transfer_class)}
            onmousedown={move |_| {
                mouse.emit((i,j, MouseEventType::MOUSEDOWN))
            }}
            onmouseenter={move |_| {
                mouse2.emit((i,j, MouseEventType::MOUSEENTER))
            }}>
            <div class="plate_cell_inner" />
        </td>
    }
}

pub fn in_rect(corner1: Option<(u8, u8)>, corner2: Option<(u8, u8)>, pt: (u8, u8)) -> bool {
    if let (Some(c1), Some(c2)) = (corner1, corner2) {
        return pt.0 <= u8::max(c1.0, c2.0)
            && pt.0 >= u8::min(c1.0, c2.0)
            && pt.1 <= u8::max(c1.1, c2.1)
            && pt.1 >= u8::min(c1.1, c2.1);
    } else {
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::in_rect;

    // in_rect tests
    #[test]
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
    fn test_in_rect4() {
        // Test cases that should fail
        let c1 = (1, 1);
        let c2 = (10, 10);
        let pt1 = (0, 0);
        let pt2 = (15, 15);
        assert_eq!(false, in_rect(Some(c1), Some(c2), pt1));
        assert_eq!(false, in_rect(Some(c1), Some(c2), pt2));
    }
}
