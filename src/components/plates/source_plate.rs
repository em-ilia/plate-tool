#![allow(non_snake_case)]

use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;

use super::super::states::NewTransferState;
use super::super::transfer_menu::RegionDisplay;

#[derive(PartialEq, Properties)]
pub struct SourcePlateProps {
    pub width: u8,
    pub height: u8,
}

#[function_component]
pub fn SourcePlate(props: &SourcePlateProps) -> Html {
    let m_start_handle: UseStateHandle<Option<(u8,u8)>> = use_state_eq(|| None);
    let m_end_handle: UseStateHandle<Option<(u8,u8)>> = use_state_eq(|| None);
    let m_stat_handle: UseStateHandle<bool> = use_state_eq(|| false);
    let m_start = m_start_handle.clone();
    let m_end = m_end_handle.clone();

    let menu_sync_callback = {
        let m_start_handle = m_start_handle.clone();
        let m_end_handle = m_end_handle.clone();
        let m_stat_handle = m_stat_handle.clone();

        move |nts: Rc<NewTransferState>| {
            log::debug!("Got an updated state!");
            if !(*m_stat_handle) {
                let pt1 = (nts.source_region.col_start, nts.source_region.row_start);
                let pt2 = (nts.source_region.col_end, nts.source_region.row_end);
                m_start_handle.set(Some(pt1));
                m_end_handle.set(Some(pt2));
            }
        }
    };
    let dispatch = Dispatch::<NewTransferState>::subscribe(menu_sync_callback);

    let mouse_callback = {
        let m_start_handle = m_start_handle.clone();
        let m_end_handle = m_end_handle.clone();
        let m_stat_handle = m_stat_handle.clone();

        Callback::from(move |(i,j,t)| {
            match t {
                MouseEventType::MOUSEDOWN => {
                    m_start_handle.set(Some((i,j)));
                    m_end_handle.set(None);
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
        let current = dispatch.get();
        m_stat_handle.set(false);
        if let Some(ul) = *m_start_handle {
            if let Some(br) = *m_end_handle {
                if let Ok(rd) = RegionDisplay::try_from((ul.0, ul.1, br.0, br.1)) {
                    dispatch.set(NewTransferState {
                        source_region: rd,
                        destination_region: current.destination_region.clone(),
                        interleave_x: current.interleave_x,
                        interleave_y: current.interleave_y })
                }
            }
        }
        })
    };

    let mouseleave_callback = Callback::clone(&mouseup_callback);

    let rows = (1..=props.height).map(|i| {
        let row = (1..=props.width).map(|j| {
            html! {
                <SourcePlateCell i={i} j={j}
                selected={in_rect(*m_start.clone(), *m_end.clone(), (i,j))}
                mouse={mouse_callback.clone()}/>
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
    color: Option<String>
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
    let _color_string = match &props.color {
        Some(c) => c.clone(),
        None => "None".to_string(),
    };
    let mouse = Callback::clone(&props.mouse);
    let mouse2 = Callback::clone(&props.mouse);
    let (i,j) = (props.i.clone(), props.j.clone());

    html! {
        <td class={classes!("plate_cell", selected_class)}
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

fn in_rect(corner1: Option<(u8, u8)>, corner2: Option<(u8, u8)>, pt: (u8, u8)) -> bool {
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
