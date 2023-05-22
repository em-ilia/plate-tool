#![allow(non_snake_case)]
use yew::prelude::*;
use super::plates::plate_container::PlateContainer;
use super::tree::Tree;
use super::transfer_menu::TransferMenu;
use super::new_plate_dialog::NewPlateDialog;

#[function_component]
pub fn MainWindow() -> Html {
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
            <PlateContainer source_dims={(24,16)} destination_dims={(24,16)}/>
            if {*new_plate_dialog_is_open} {
            <NewPlateDialog close_callback={new_plate_dialog_callback}/>
            }
        </div>
    }
}
