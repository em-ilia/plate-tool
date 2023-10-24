#![allow(non_snake_case)]
use std::collections::HashSet;

use js_sys::Array;
use lazy_static::lazy_static;
use regex::Regex;
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use web_sys::{
    Blob, HtmlAnchorElement, HtmlButtonElement, HtmlDialogElement, HtmlFormElement,
    HtmlInputElement, HtmlOptionElement, HtmlSelectElement, Url,
};
use yew::prelude::*;
use yewdux::prelude::*;

use super::new_plate_dialog::NewPlateDialog;
use super::plates::plate_container::PlateContainer;
use super::states::{CurrentTransfer, MainState};
use super::transfer_menu::{letters_to_num, RegionDisplay, TransferMenu};
use super::tree::Tree;

use crate::data::csv::state_to_csv;
use crate::data::plate_instances::PlateInstance;
use crate::data::transfer::Transfer;
use crate::data::transfer_region::{Region, TransferRegion};

#[function_component]
pub fn MainWindow() -> Html {
    let (main_state, main_dispatch) = use_store::<MainState>();
    let (_, ct_dispatch) = use_store::<CurrentTransfer>();

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
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let confirm =
                window.confirm_with_message("This will reset all plates and transfers. Proceed?");
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
            if main_state.transfers.is_empty() {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("No transfers to export.")
                    .unwrap();
                return;
            }
            web_sys::window().unwrap().alert_with_message("CSV export is currently not importable. Export as JSON if you'd like to back up your work!").unwrap();
            if let Ok(csv) = state_to_csv(&main_state) {
                save_str(&csv, "transfers.csv");
            }
        })
    };

    let export_json_button_callback = {
        Callback::from(move |_| {
            if let Ok(json) = serde_json::to_string(&main_state) {
                save_str(&json, "plate-tool-state.json");
            } else {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("Failed to export.")
                    .unwrap();
            }
        })
    };

    let import_json_button_callback = {
        let main_dispatch = main_dispatch.clone();
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            let modal = document
                .create_element("dialog")
                .unwrap()
                .dyn_into::<HtmlDialogElement>()
                .unwrap();
            modal.set_text_content(Some("Import File:"));
            let onclose_callback = {
                let modal = modal.clone();
                Closure::<dyn FnMut(_)>::new(move |_: Event| {
                    modal.remove();
                })
            };
            modal.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();

            let form = document
                .create_element("form")
                .unwrap()
                .dyn_into::<HtmlFormElement>()
                .unwrap();
            let input = document
                .create_element("input")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            input.set_type("file");
            input.set_accept(".json");
            form.append_child(&input).unwrap();

            let input_callback = {
                let main_dispatch = main_dispatch.clone();
                let modal = modal.clone();
                Closure::<dyn FnMut(_)>::new(move |e: Event| {
                    if let Some(input) = e.current_target() {
                        let input = input
                            .dyn_into::<HtmlInputElement>()
                            .expect("We know this is an input.");
                        if let Some(files) = input.files() {
                            if let Some(file) = files.get(0) {
                                let fr = web_sys::FileReader::new().unwrap();
                                fr.read_as_text(&file).unwrap();
                                let fr1 = fr.clone(); // Clone to avoid outliving closure
                                let main_dispatch = main_dispatch.clone(); // Clone to satisfy FnMut
                                                                           // trait
                                let modal = modal.clone();
                                let onload = Closure::<dyn FnMut(_)>::new(move |_: Event| {
                                    if let Some(value) =
                                        &fr1.result().ok().and_then(|v| v.as_string())
                                    {
                                        let ms = serde_json::from_str::<MainState>(value);
                                        match ms {
                                            Ok(ms) => main_dispatch.set(ms),
                                            Err(e) => log::debug!("{:?}", e),
                                        };
                                        modal.close();
                                    }
                                });
                                fr.set_onload(Some(onload.as_ref().unchecked_ref()));
                                onload.forget(); // Magic (don't touch)
                            }
                        }
                    }
                })
            };
            input.set_onchange(Some(input_callback.as_ref().unchecked_ref()));
            input_callback.forget(); // Magic straight from the docs, don't touch :(

            modal.append_child(&form).unwrap();
            body.append_child(&modal).unwrap();
            modal.show_modal().unwrap();
        })
    };

    let import_transfer_csv_callback = {
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();
            let modal = document
                .create_element("dialog")
                .unwrap()
                .dyn_into::<HtmlDialogElement>()
                .unwrap();
            modal.set_text_content(Some("Import File:"));
            let onclose_callback = {
                let modal = modal.clone();
                Closure::<dyn FnMut(_)>::new(move |_: Event| {
                    modal.remove();
                })
            };
            modal.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();

            let form = document
                .create_element("form")
                .unwrap()
                .dyn_into::<HtmlFormElement>()
                .unwrap();
            let input = document
                .create_element("input")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            input.set_type("file");
            input.set_accept(".csv");
            form.append_child(&input).unwrap();

            let input_callback = {
                let main_dispatch = main_dispatch.clone();
                let modal = modal.clone();
                Closure::<dyn FnMut(_)>::new(move |e: Event| {
                    if let Some(input) = e.current_target() {
                        let input = input
                            .dyn_into::<HtmlInputElement>()
                            .expect("We know this is an input.");
                        if let Some(files) = input.files() {
                            if let Some(file) = files.get(0) {
                                let fr = web_sys::FileReader::new().unwrap();
                                fr.read_as_text(&file).unwrap();
                                let fr1 = fr.clone(); // Clone to avoid outliving closure
                                let main_dispatch = main_dispatch.clone(); // Clone to satisfy FnMut
                                                                           // trait
                                let modal = modal.clone();
                                let onload = Closure::<dyn FnMut(_)>::new(move |_: Event| {
                                    if let Some(value) =
                                        &fr1.result().ok().and_then(|v| v.as_string())
                                    {
                                        let mut rdr = csv::Reader::from_reader(value.as_bytes());
                                        let mut records = Vec::new();
                                        for record in
                                            rdr.deserialize::<crate::data::csv::TransferRecord>()
                                        {
                                            match record {
                                                Ok(r) => {
                                                    //log::debug!("{:?}", r);
                                                    records.push(r);
                                                }
                                                Err(e) => {
                                                    log::debug!("{:?}", e);
                                                }
                                            }
                                        }

                                        let mut sources: HashSet<String> = HashSet::new();
                                        let mut destinations: HashSet<String> = HashSet::new();
                                        for record in records.iter() {
                                            sources.insert(record.source_plate.clone());
                                            destinations.insert(record.destination_plate.clone());
                                        }

                                        let window = web_sys::window().unwrap();
                                        let document = window.document().unwrap();
                                        let form = document
                                            .create_element("form")
                                            .unwrap()
                                            .dyn_into::<HtmlFormElement>()
                                            .unwrap();
                                        let from_source = document
                                            .create_element("select")
                                            .unwrap()
                                            .dyn_into::<HtmlSelectElement>()
                                            .unwrap();
                                        for source in sources {
                                            let option = document
                                                .create_element("option")
                                                .unwrap()
                                                .dyn_into::<HtmlOptionElement>()
                                                .unwrap();
                                            option.set_value(&source);
                                            option.set_text(&source);
                                            from_source.append_child(&option).unwrap();
                                        }
                                        let to_source = document
                                            .create_element("select")
                                            .unwrap()
                                            .dyn_into::<HtmlSelectElement>()
                                            .unwrap();
                                        for source in &main_dispatch.get().source_plates {
                                            let option = document
                                                .create_element("option")
                                                .unwrap()
                                                .dyn_into::<HtmlOptionElement>()
                                                .unwrap();
                                            option.set_value(&source.name);
                                            option.set_text(&source.name);
                                            to_source.append_child(&option).unwrap();
                                        }
                                        let from_dest = document
                                            .create_element("select")
                                            .unwrap()
                                            .dyn_into::<HtmlSelectElement>()
                                            .unwrap();
                                        for dest in destinations {
                                            let option = document
                                                .create_element("option")
                                                .unwrap()
                                                .dyn_into::<HtmlOptionElement>()
                                                .unwrap();
                                            option.set_value(&dest);
                                            option.set_text(&dest);
                                            from_dest.append_child(&option).unwrap();
                                        }
                                        let to_dest = document
                                            .create_element("select")
                                            .unwrap()
                                            .dyn_into::<HtmlSelectElement>()
                                            .unwrap();
                                        for dest in &main_dispatch.get().destination_plates {
                                            let option = document
                                                .create_element("option")
                                                .unwrap()
                                                .dyn_into::<HtmlOptionElement>()
                                                .unwrap();
                                            option.set_value(&dest.name);
                                            option.set_text(&dest.name);
                                            to_dest.append_child(&option).unwrap();
                                        }
                                        let submit = document
                                            .create_element("button")
                                            .unwrap()
                                            .dyn_into::<HtmlButtonElement>()
                                            .unwrap();
                                        submit.set_value("Submit");
                                        let submit_callback = {
                                            let main_dispatch = main_dispatch.clone();
                                            let from_source = from_source.clone();
                                            let to_source = to_source.clone();
                                            let from_dest = from_dest.clone();
                                            let to_dest = to_dest.clone();
                                            Closure::<dyn FnMut(_)>::new(move |_: Event| {
                                                let from_source = from_source.value();
                                                let to_source = to_source.value();
                                                let from_dest = from_dest.value();
                                                let to_dest = to_dest.value();

                                                lazy_static! {
                                                    static ref REGEX: Regex =
                                                        Regex::new(r"([A-Z]+)(\d+)").unwrap();
                                                }
                                                let records: Vec<((u8, u8), (u8, u8))> = records
                                                    .iter()
                                                    .filter(|record| {
                                                        record.source_plate == from_source
                                                    })
                                                    .filter(|record| {
                                                        record.destination_plate == from_dest
                                                    })
                                                    .map(|record| {
                                                        let c1 = REGEX
                                                            .captures(&record.source_well)
                                                            .unwrap();
                                                        let c2 = REGEX
                                                            .captures(&record.destination_well)
                                                            .unwrap();
                                                        log::debug!("{} {}", &record.source_well, &record.destination_well);
                                                        log::debug!("{},{}  {},{}", &c1[1], &c1[2], &c2[1], &c2[2]);

                                                        (
                                                            (
                                                                letters_to_num(&c1[1]).unwrap(),
                                                                c1[2].parse::<u8>().unwrap(),
                                                            ),
                                                            (
                                                                letters_to_num(&c2[1]).unwrap(),
                                                                c2[2].parse::<u8>().unwrap(),
                                                            ),
                                                        )
                                                    })
                                                    .collect();

                                                let spi = main_dispatch
                                                    .get()
                                                    .source_plates
                                                    .iter()
                                                    .find(|src| src.name == to_source)
                                                    .unwrap()
                                                    .clone();
                                                let dpi = main_dispatch
                                                    .get()
                                                    .destination_plates
                                                    .iter()
                                                    .find(|dest| dest.name == to_dest)
                                                    .unwrap()
                                                    .clone();

                                                let custom_region = Region::new_custom(&records);
                                                let transfer_region = TransferRegion {
                                                    source_region: custom_region.clone(),
                                                    dest_region: custom_region,
                                                    interleave_source: (1, 1),
                                                    interleave_dest: (1, 1),
                                                    source_plate: spi.plate,
                                                    dest_plate: dpi.plate,
                                                };

                                                let transfer = Transfer::new(
                                                    spi,
                                                    dpi,
                                                    transfer_region,
                                                    "Custom Transfer".to_string(),
                                                );
                                                main_dispatch.reduce_mut(|state| {
                                                    state.transfers.push(transfer);
                                                    state.selected_transfer = state
                                .transfers
                                .last()
                                .expect("An element should have just been added")
                                .get_uuid();
                                                });
                                            })
                                        };
                                        submit.set_onclick(Some(
                                            submit_callback.as_ref().unchecked_ref(),
                                        ));
                                        submit_callback.forget();

                                        form.append_child(&from_source).unwrap();
                                        form.append_child(&to_source).unwrap();
                                        form.append_child(&from_dest).unwrap();
                                        form.append_child(&to_dest).unwrap();
                                        modal.append_child(&submit).unwrap();
                                        modal.append_child(&form).unwrap();
                                    }
                                });
                                fr.set_onload(Some(onload.as_ref().unchecked_ref()));
                                onload.forget(); // Magic (don't touch)
                            }
                        }
                    }
                })
            };
            input.set_onchange(Some(input_callback.as_ref().unchecked_ref()));
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
                <div class="dropdown-sub">
                    <button>{"Import"}</button>
                    <div>
                        <button onclick={import_json_button_callback}>{"Import from JSON"}</button>
                        <button onclick={import_transfer_csv_callback}>{"Import Transfer from CSV"}</button>
                    </div>
                </div>
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
    let blob =
        Blob::new_with_str_sequence(&Array::from_iter(std::iter::once(JsValue::from_str(data))));
    if let Ok(blob) = blob {
        let url = Url::create_object_url_with_blob(&blob).expect("We have a blob, why not URL?");
        // Beneath is the cool hack to download files
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let anchor = document
            .create_element("a")
            .unwrap()
            .dyn_into::<HtmlAnchorElement>()
            .unwrap();
        anchor.set_download(name);
        anchor.set_href(&url);
        anchor.click();
    }
}
