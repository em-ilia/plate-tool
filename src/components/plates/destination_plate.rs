#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;
use std::rc::Rc;

use crate::data::plate_instances::PlateInstance;
use crate::data::transfer_region::{TransferRegion, Region};
use crate::components::states::{CurrentTransfer};

use super::super::transfer_menu::RegionDisplay;

#[derive(Properties, PartialEq)]
pub struct DestinationPlateProps {
    pub source_plate: PlateInstance,
    pub destination_plate: PlateInstance,
}

#[function_component]
pub fn DestinationPlate(props: &DestinationPlateProps) -> Html {
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();
    let m_start_handle: UseStateHandle<Option<(u8,u8)>> = use_state_eq(|| None);
    let m_end_handle: UseStateHandle<Option<(u8,u8)>> = use_state_eq(|| None);
    let m_stat_handle: UseStateHandle<bool> = use_state_eq(|| false);
    let m_start = m_start_handle.clone();
    let m_end = m_end_handle.clone();

    if !(*m_stat_handle) {
        let (pt1, pt2) = match ct_state.transfer.dest_region {
            Region::Point((x,y)) => ((x,y),(x,y)),
            Region::Rect(c1, c2) => (c1, c2),
        };
        m_start_handle.set(Some(pt1));
        m_end_handle.set(Some(pt2));
    }
    let destination_wells = ct_state.transfer.get_destination_wells();

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
                            state.transfer.dest_region = Region::from(&rd);
                        });
                    }
                }
            }
        })
    };

    let mouseleave_callback = Callback::clone(&mouseup_callback);

    let rows = (1..=props.destination_plate.plate.size().0).map(|i| {
        let row = (1..=props.destination_plate.plate.size().1).map(|j| {
            html! {
                <DestPlateCell i={i} j={j} 
                selected={super::source_plate::in_rect(*m_start.clone(), *m_end.clone(), (i,j))} 
                mouse={mouse_callback.clone()}
                in_transfer={destination_wells.contains(&(i,j))}
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
    pub in_transfer: Option<bool>,
}

#[function_component]
fn DestPlateCell(props: &DestPlateCellProps) -> Html {
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
