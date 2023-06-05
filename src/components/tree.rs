#![allow(non_snake_case)]

use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlDialogElement, HtmlElement};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::states::{CurrentTransfer, MainState};
use crate::components::transfer_menu::RegionDisplay;
use crate::data::transfer_region::Region;

#[derive(PartialEq, Properties)]
pub struct TreeProps {
    pub open_new_plate_callback: Callback<()>,
}

#[function_component]
pub fn Tree(props: &TreeProps) -> Html {
    let (main_state, main_dispatch) = use_store::<MainState>();
    let (ct_state, ct_dispatch) = use_store::<CurrentTransfer>();
    let plate_modal_id: UseStateHandle<Option<Uuid>> = use_state(|| None);

    let open_plate_info_callback = {
        let plate_menu_id = plate_modal_id.clone();
        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let li = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
            if let Some(li) = li {
                if let Ok(id) = u128::from_str_radix(li.id().as_str(), 10) {
                    plate_menu_id.set(Some(Uuid::from_u128(id)));
                }
            }
        })
    };
    let plate_info_close_callback = {
        let plate_menu_id = plate_modal_id.clone();
        Callback::from(move |_| {
            plate_menu_id.set(None);
        })
    };
    let plate_info_delete_callback = {
        let dispatch = main_dispatch.clone();
        let plate_menu_id = plate_modal_id.clone();
        Callback::from(move |_| {
            if let Some(id) = *plate_menu_id {
                dispatch.reduce_mut(|state| {
                    state.del_plate(id);
                });
            }
        })
    };
    let source_plate_select_callback = {
        let main_dispatch = main_dispatch.clone();
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let li = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
            if let Some(li) = li {
                if let Ok(id) = u128::from_str_radix(li.id().as_str(), 10) {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.transfer_region.source_region = Region::default();
                        state.transfer.transfer_region.dest_region = Region::default();
                    });
                    main_dispatch.reduce_mut(|state| {
                        state.selected_source_plate = Uuid::from_u128(id);
                        state.selected_transfer = Uuid::nil();
                    });
                }
            }
        })
    };
    let destination_plate_select_callback = {
        let main_dispatch = main_dispatch.clone();
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let li = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
            if let Some(li) = li {
                if let Ok(id) = u128::from_str_radix(li.id().as_str(), 10) {
                    ct_dispatch.reduce_mut(|state| {
                        state.transfer.transfer_region.source_region = Region::default();
                        state.transfer.transfer_region.dest_region = Region::default();
                    });
                    main_dispatch.reduce_mut(|state| {
                        state.selected_dest_plate = Uuid::from_u128(id);
                        state.selected_transfer = Uuid::nil();
                    });
                }
            }
        })
    };

    let transfer_select_callback = {
        let main_state = main_state.clone();
        let main_dispatch = main_dispatch.clone();
        let ct_dispatch = ct_dispatch.clone();

        Callback::from(move |e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let li = target.and_then(|t| t.dyn_into::<HtmlElement>().ok());
            if let Some(li) = li {
                if let Ok(id) = u128::from_str_radix(li.id().as_str(), 10) {
                    let id = Uuid::from_u128(id);
                    if let Some(transfer) = main_state.transfers
                                            .iter().find(|transfer| transfer.get_uuid() == id) {

                        main_dispatch.reduce_mut(|state| {
                            state.selected_source_plate = transfer.source_id;
                            state.selected_dest_plate = transfer.dest_id;
                            state.selected_transfer = id;
                        });
                        ct_dispatch.reduce_mut(|state| {
                            state.transfer = transfer.clone();
                        });
                    }
                }
            }
        })
    };

    let source_plates = main_state
        .source_plates
        .iter()
        .map(|spi| {
            html! { <li id={spi.get_uuid().as_u128().to_string()}
            ondblclick={open_plate_info_callback.clone()}
            onclick={source_plate_select_callback.clone()}
            class={classes!(
                if spi.get_uuid() == main_state.selected_source_plate {Some("selected")}
                     else {None}
            )}>
                {String::from(spi)}
                </li> }
        })
        .collect::<Html>();
    let dest_plates = main_state
        .destination_plates
        .iter()
        .map(|dpi| {
            html! { <li id={dpi.get_uuid().as_u128().to_string()}
            ondblclick={open_plate_info_callback.clone()}
            onclick={destination_plate_select_callback.clone()}
            class={classes!(
                if dpi.get_uuid() == main_state.selected_dest_plate {Some("selected")}
                     else {None}
            )}> {String::from(dpi)} </li> }
        })
        .collect::<Html>();
    let transfers = main_state
        .transfers
        .iter()
        .map(|transfer| {
            html! { <li id={transfer.get_uuid().as_u128().to_string()}
                     onclick={transfer_select_callback.clone()}
                     class={classes!(
                         if transfer.get_uuid() == main_state.selected_transfer {Some("selected")}
                         else {None})}>
                {transfer.name.clone()}
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <div class="tree">
            <div id="source-plates">
            <h3>{"Source Plates:"}</h3>
            <ul>
                {source_plates}
            </ul>
            </div>
            <div id="destination-plates">
            <h3>{"Destination Plates:"}</h3>
            <ul>
                {dest_plates}
            </ul>
            </div>
            <div id="transfers">
            <h3>{"Transfers:"}</h3>
            <ul>
                {transfers}
            </ul>
            </div>
            if let Some(id) = *plate_modal_id {
                <PlateInfoModal id={id} dialog_close_callback={plate_info_close_callback}
                delete_button_callback={plate_info_delete_callback}/>
            }

            <div id="controls">
            <button type="button"
            onclick={
                let open_new_plate_callback = props.open_new_plate_callback.clone();
                move |_| {open_new_plate_callback.emit(())}
            }>
            {"New Plate"}</button>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct PlateInfoModalProps {
    id: Uuid,
    dialog_close_callback: Callback<()>,
    delete_button_callback: Callback<()>,
}

#[function_component]
fn PlateInfoModal(props: &PlateInfoModalProps) -> Html {
    let (state, dispatch) = use_store::<MainState>();
    let dialog_ref = use_node_ref();

    let mut plate = state
        .source_plates
        .iter()
        .find(|spi| spi.get_uuid() == props.id);
    if plate == None {
        plate = state
            .destination_plates
            .iter()
            .find(|dpi| dpi.get_uuid() == props.id);
    }
    let plate_name = match plate {
        Some(plate) => plate.name.clone(),
        None => "Not Found".to_string(),
    };
    let onclose = {
        let dialog_close_callback = props.dialog_close_callback.clone();
        move |_| dialog_close_callback.emit(())
    };

    let delete_onclick = {
        let delete_button_callback = props.delete_button_callback.clone();
        let dialog_ref = dialog_ref.clone();
        move |_| {
            delete_button_callback.emit(());
            dialog_ref.cast::<HtmlDialogElement>().unwrap().close();
        }
    };

    {
        let dialog_ref = dialog_ref.clone();

        use_effect_with_deps(
            |dialog_ref| {
                dialog_ref
                    .cast::<HtmlDialogElement>()
                    .unwrap()
                    .show_modal()
                    .ok();
            },
            dialog_ref,
        );
    }

    html! {
        <dialog ref={dialog_ref} class="dialog" onclose={onclose}>
            <h2>{"Plate Info"}</h2>
            <h3>{"Name: "}<input type="text" value={plate_name} /></h3>
            <button onclick={delete_onclick}>{"Delete"}</button>
        </dialog>
    }
}
