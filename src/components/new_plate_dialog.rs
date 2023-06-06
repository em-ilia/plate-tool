use yew::prelude::*;
use yewdux::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, FormData, HtmlDialogElement, HtmlFormElement};

use crate::components::states::MainState;
use crate::data::plate::*;
use crate::data::{plate_instances::PlateInstance, transfer::Transfer};

#[derive(PartialEq, Properties)]
pub struct NewPlateDialogProps {
    pub close_callback: Callback<()>,
}

#[function_component]
pub fn NewPlateDialog(props: &NewPlateDialogProps) -> Html {
    let (state, dispatch) = use_store::<MainState>();

    let new_plate_callback = {
        let dispatch = dispatch.clone();
        let close_callback = props.close_callback.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            close_callback.emit(());
            let target: Option<EventTarget> = e.target();
            let form = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok());
            if let Some(form) = form {
                if let Ok(form_data) = FormData::new_with_form(&form) {
                    let name = form_data.get("new_plate_name").as_string().unwrap();
                    let format = match form_data.get("plate_format").as_string().unwrap().as_str() {
                        "384" => PlateFormat::W384,
                        "96" => PlateFormat::W96,
                        _ => PlateFormat::W6,
                    };
                    if let Some(pt_string) = form_data.get("new_plate_type").as_string() {
                        let plate_type = match pt_string.as_str() {
                            "src" => PlateType::Source,
                            "dest" => PlateType::Destination,
                            _ => PlateType::Source,
                        };
                        dispatch.reduce_mut(|s| {
                            if plate_type == PlateType::Source {
                                s.add_source_plate(PlateInstance::new(
                                    PlateType::Source,
                                    format,
                                    name,
                                ))
                            } else {
                                s.add_dest_plate(PlateInstance::new(
                                    PlateType::Destination,
                                    format,
                                    name,
                                ))
                            }
                        });
                    }
                }
            }
        })
    };

    let onclose = {
        let close_callback = props.close_callback.clone();
        Callback::from(move |e: Event| {
            close_callback.emit(());
        })
    };

    // This whole section is optional, only if you want the backdrop
    let dialog_ref = use_node_ref();
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
        <dialog ref={dialog_ref} class="dialog new_plate_dialog" onclose={onclose}>
            <h2>{"Create a plate:"}</h2>
            <form onsubmit={new_plate_callback}>
            <input type="text" name="new_plate_name" placeholder="Name"/>
            <select name="plate_format">
                <option value="96">{"96"}</option>
                <option value="384">{"384"}</option>
            </select>
            <input type="radio" name="new_plate_type" id="npt_src" value="src" />
            <label for="npt_src">{"Source"}</label>
            <input type="radio" name="new_plate_type" id="npt_dest" value="dest" />
            <label for="npt_dest">{"Destination"}</label>
            <input type="submit" name="new_plate_button" value="Create" />
            </form>
        </dialog>
    }
}

impl From<&PlateInstance> for String {
    fn from(value: &PlateInstance) -> Self {
        // Could have other formatting here
        format!("{}, {}", value.name, value.plate.plate_format)
    }
}
