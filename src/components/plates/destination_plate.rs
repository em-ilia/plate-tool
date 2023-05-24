#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;

use crate::data::plate_instances::PlateInstance;

use super::super::states::NewTransferState;
use super::super::transfer_menu::RegionDisplay;

#[derive(Properties, PartialEq)]
pub struct DestinationPlateProps {
    pub plate: PlateInstance,
}

#[function_component]
pub fn DestinationPlate(props: &DestinationPlateProps) -> Html {
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
            if !(*m_stat_handle) {
                let pt1 = (nts.destination_region.col_start, nts.destination_region.row_start);
                let pt2 = (nts.destination_region.col_end, nts.destination_region.row_end);
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
            let current = dispatch.get();
            m_stat_handle.set(false);
            if let Some(ul) = *m_start_handle {
                if let Some(br) = *m_end_handle {
                    if let Ok(rd) = RegionDisplay::try_from((ul.0, ul.1, br.0, br.1)) {
                        dispatch.reduce_mut(|state| {
                            state.destination_region = rd;
                        });
                    }
                }
            }
        })
    };

    let mouseleave_callback = Callback::clone(&mouseup_callback);

    let rows = (1..=props.plate.plate.size().1).map(|i| {
        let row = (1..=props.plate.plate.size().0).map(|j| {
            html! {
                <DestPlateCell i={i} j={j} 
                selected={super::source_plate::in_rect(*m_start.clone(), *m_end.clone(), (i,j))} 
                mouse={mouse_callback.clone()} />
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

#[derive(Debug)]
pub enum MouseEventType {
    MOUSEDOWN,
    MOUSEENTER
}

#[derive(Properties, PartialEq)]
pub struct DestPlateCellProps {
    pub i: u8,
    pub j: u8,
    pub selected: bool,
    pub mouse: Callback<(u8,u8, MouseEventType)>,
    pub color: Option<String>
}

#[function_component]
fn DestPlateCell(props: &DestPlateCellProps) -> Html {
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
