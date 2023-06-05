#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::{JsValue, JsCast, prelude::*};
use web_sys::{Blob, Url, HtmlAnchorElement, HtmlDialogElement, HtmlInputElement, HtmlFormElement};
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

    let new_button_callback = {
        let main_dispatch = main_dispatch.clone();
        let ct_dispatch = ct_dispatch.clone();
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let confirm = window.confirm_with_message("This will reset all plates and transfers. Proceed?");
            if let Ok(confirm) = confirm {
                if confirm {
                    main_dispatch.set(MainState::default());
                    ct_dispatch.set(CurrentTransfer::default());
                }
            }
        })
    };

    let export_csv_button_callback = {
        let main_state = main_state.clone();
        Callback::from(move |_| {
            if main_state.transfers.len() == 0 {
                web_sys::window().unwrap().alert_with_message("No transfers to export.").unwrap();
                return ()
            }
            web_sys::window().unwrap().alert_with_message("CSV export is currently not importable. Export as JSON if you'd like to back up your work!").unwrap();
            if let Ok(csv) = state_to_csv(&main_state) {
                save_str(&csv, "transfers.csv");
            }
        })
    };

    let export_json_button_callback = {
        let main_state = main_state.clone();
        Callback::from(move |_| {
            if let Ok(json) = serde_json::to_string(&main_state) {
                save_str(&json, "plate-tool-state.json");
            } else {
                web_sys::window().unwrap().alert_with_message("Failed to export.").unwrap();
            }
        })
    };

    let import_json_button_callback = {
        let main_dispatch = main_dispatch.clone();
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            let modal = document.create_element("dialog").unwrap().dyn_into::<HtmlDialogElement>().unwrap();
            modal.set_text_content(Some("Import File:"));
            let onclose_callback = {
                let modal = modal.clone();
                Closure::<dyn FnMut(_)>::new(move |_: Event| {
                    modal.remove();
                })
            };
            modal.set_onclose(Some(&onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();

            let form = document.create_element("form").unwrap().dyn_into::<HtmlFormElement>().unwrap();
            let input = document.create_element("input").unwrap().dyn_into::<HtmlInputElement>().unwrap();
            input.set_type("file");
            input.set_accept(".json");
            form.append_child(&input).unwrap();

            let input_callback = {
                let main_dispatch = main_dispatch.clone();
                let modal = modal.clone();
                Closure::<dyn FnMut(_)>::new(move |e: Event| {
                if let Some(input) = e.current_target() {
                    let input = input.dyn_into::<HtmlInputElement>().expect("We know this is an input.");
                    if let Some(files) = input.files() {
                        if let Some(file) = files.get(0) {
                            let fr = web_sys::FileReader::new().unwrap();
                            fr.read_as_text(&file).unwrap();
                            let fr1 = fr.clone(); // Clone to avoid outliving closure
                            let main_dispatch = main_dispatch.clone(); // Clone to satisfy FnMut
                                                                       // trait
                            let modal = modal.clone();
                            let onload = Closure::<dyn FnMut(_)>::new(move |e: Event| {
                                log::debug!("{:?}",&fr1.result());
                                if let Some(value) = &fr1.result().ok().and_then(|v| v.as_string()) {
                                    let ms = serde_json::from_str::<MainState>(value);
                                    match ms {
                                        Ok(ms) => main_dispatch.set(ms),
                                        Err(e) => log::debug!("{:?}", e),
                                    };
                                    modal.close();
                                }
                            });
                            fr.set_onload(Some(&onload.as_ref().unchecked_ref()));
                            onload.forget(); // Magic (don't touch)
                        }
                    }
                }
            })};
            input.set_onchange(Some(&input_callback.as_ref().unchecked_ref()));
            input_callback.forget(); // Magic straight from the docs, don't touch :(

            modal.append_child(&form).unwrap();
            body.append_child(&modal).unwrap();
            modal.show_modal().unwrap();
        })
    };

    html! {
        <>
        <div class="upper_menu">
            <div class="dropdown">
                <button>{"File"}</button>
                <button onclick={new_button_callback}>{"New"}</button>
                <div class="dropdown-sub">
                    <button>{"Export"}</button>
                    <div>
                        <button onclick={export_csv_button_callback}>{"Export as CSV"}</button>
                        <button onclick={export_json_button_callback}>{"Export as JSON"}</button>
                    </div>
                </div>
                <button onclick={import_json_button_callback}>{"Import"}</button>
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

fn save_str(data: &str, name: &str) {
    let blob = Blob::new_with_str_sequence(
                    &Array::from_iter(std::iter::once(JsValue::from_str(data))));
    if let Ok(blob) = blob {
        let url = Url::create_object_url_with_blob(&blob).expect("We have a blob, why not URL?");
        // Beneath is the cool hack to download files
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let anchor = document.create_element("a").unwrap()
                             .dyn_into::<HtmlAnchorElement>().unwrap();
        anchor.set_download(name);
        anchor.set_href(&url);
        anchor.click();
    }
}
