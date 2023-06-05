#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{Blob, Url, HtmlAnchorElement};
use js_sys::Array;

use super::new_plate_dialog::NewPlateDialog;
use super::plates::plate_container::PlateContainer;
use super::states::{CurrentTransfer, MainState};
use super::transfer_menu::TransferMenu;
use super::tree::Tree;

use crate::data::plate_instances::PlateInstance;
use crate::data::csv::state_to_csv;

#[function_component]
pub fn MainWindow() -> Html {
    let (main_state, main_dispatch) = use_store::<MainState>();
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();

    let source_plate_instance: Option<PlateInstance> = main_state
        .source_plates
        .iter()
        .find(|spi| spi.get_uuid() == main_state.selected_source_plate)
        .cloned();
    if let Some(spi) = source_plate_instance.clone() {
        ct_dispatch.reduce_mut(|state| {
            state.transfer.transfer_region.source_plate = spi.plate;
        });
    }
    let destination_plate_instance: Option<PlateInstance> = main_state
        .destination_plates
        .iter()
        .find(|dpi| dpi.get_uuid() == main_state.selected_dest_plate)
        .cloned();
    if let Some(dpi) = destination_plate_instance.clone() {
        ct_dispatch.reduce_mut(|state| {
            state.transfer.transfer_region.dest_plate = dpi.plate;
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

    let save_button_callback = {
        let main_state = main_state.clone();
        Callback::from(move |_| {
            if let Ok(csv) = state_to_csv(&main_state) {
                let csv: &str = &csv;
                let blob = Blob::new_with_str_sequence(
                                &Array::from_iter(std::iter::once(JsValue::from_str(csv))));
                if let Ok(blob) = blob {
                    let url = Url::create_object_url_with_blob(&blob).expect("We have a blob, why not URL?");
                    // Beneath is the cool hack to download files
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let anchor = document.create_element("a").unwrap()
                                         .dyn_into::<HtmlAnchorElement>().unwrap();
                    anchor.set_download("transfers.csv");
                    anchor.set_href(&url);
                    anchor.click();
                }
            }
        })
    };

    html! {
        <>
        <div class="upper_menu">
            <div class="dropdown">
                <button>{"File"}</button>
                <button onclick={save_button_callback}>{"Save"}</button>
            </div>
        </div>
        <div class="main_container">
            <Tree open_new_plate_callback={open_new_plate_dialog_callback}/>
            <TransferMenu />
            <PlateContainer source_dims={source_plate_instance}
             destination_dims={destination_plate_instance}/>
            if {*new_plate_dialog_is_open} {
            <NewPlateDialog close_callback={new_plate_dialog_callback}/>
            }
        </div>
        </>
    }
}
