#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;

use super::states::{MainState, CurrentTransfer};
use super::plates::plate_container::PlateContainer;
use super::tree::Tree;
use super::transfer_menu::TransferMenu;
use super::new_plate_dialog::NewPlateDialog;

use crate::data::plate_instances::PlateInstance;

#[function_component]
pub fn MainWindow() -> Html {
    let (main_state, main_dispatch) = use_store::<MainState>();
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();

    let source_plate_instance: Option<PlateInstance> = main_state.source_plates.iter()
        .find(|spi| {spi.get_uuid() == main_state.selected_source_plate})
        .cloned();
    if let Some(spi) = source_plate_instance.clone() {
    ct_dispatch.reduce_mut(|state| {
        state.transfer.source_plate = spi.plate;
        });
    }
    let destination_plate_instance: Option<PlateInstance> = main_state.destination_plates.iter()
        .find(|dpi| {dpi.get_uuid() == main_state.selected_dest_plate})
        .cloned();
    if let Some(dpi) = destination_plate_instance.clone() {
    ct_dispatch.reduce_mut(|state| {
        state.transfer.dest_plate = dpi.plate;
        });
    }

    let new_plate_dialog_is_open = use_state_eq(|| false);
    let new_plate_dialog_callback = {
        let new_plate_dialog_is_open = new_plate_dialog_is_open.clone();
        Callback::from(move |_| {
            new_plate_dialog_is_open.set(false);
        })
    };
    let open_new_plate_dialog_callback = {
        let new_plate_dialog_is_open = new_plate_dialog_is_open.clone();
        Callback::from(move |_| {
            new_plate_dialog_is_open.set(true);
        })
    };

    html!{
        <div class="main_container">
            <Tree open_new_plate_callback={open_new_plate_dialog_callback}/>
            <TransferMenu />
            <PlateContainer source_dims={source_plate_instance}
             destination_dims={destination_plate_instance}/>
            if {*new_plate_dialog_is_open} {
            <NewPlateDialog close_callback={new_plate_dialog_callback}/>
            }
        </div>
    }
}
