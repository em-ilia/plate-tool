#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;

use super::states::{MainState, NewTransferState};
use super::plates::plate_container::PlateContainer;
use super::tree::Tree;
use super::transfer_menu::TransferMenu;
use super::new_plate_dialog::NewPlateDialog;

use crate::data::plate_instances::PlateInstance;

#[function_component]
pub fn MainWindow() -> Html {
    let (main_state, main_dispatch) = use_store::<MainState>();
    let (selection_state, selection_dispatch) = use_store::<NewTransferState>();

    let source_plate_instance = main_state.source_plates.iter()
        .find(|spi| {spi.get_uuid() == selection_state.source_id});
    let source_dims = match source_plate_instance {
        Some(spi) => Some(spi.plate.size()),
        None => None,
    };
    let destination_plate_instance = main_state.destination_plates.iter()
        .find(|dpi| {dpi.get_uuid() == selection_state.destination_id});
    let destination_dims = match destination_plate_instance {
        Some(dpi) => Some(dpi.plate.size()),
        None => None,
    };

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
            <PlateContainer source_dims={source_dims} destination_dims={destination_dims}/>
            if {*new_plate_dialog_is_open} {
            <NewPlateDialog close_callback={new_plate_dialog_callback}/>
            }
        </div>
    }
}
