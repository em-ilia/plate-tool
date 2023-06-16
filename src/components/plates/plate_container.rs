#![allow(non_snake_case)]
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::data::plate_instances::PlateInstance;

use super::destination_plate::DestinationPlate;
use super::source_plate::SourcePlate;

#[derive(Properties, PartialEq)]
pub struct PlateContainerProps {
    pub source_dims: Option<PlateInstance>,
    pub destination_dims: Option<PlateInstance>,
}

#[function_component]
pub fn PlateContainer(props: &PlateContainerProps) -> Html {
    let cell_height = {
        let height = web_sys::window()
            .unwrap()
            .inner_height()
            .unwrap()
            .as_f64()
            .unwrap();
        let width = web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap();
        if let (Some(src_d), Some(dest_d)) = (&props.source_dims, &props.destination_dims) {
            let h = (0.78 * height) / (src_d.plate.size().0 + dest_d.plate.size().0) as f64;
            let w = (0.90 * width) / (src_d.plate.size().1 + dest_d.plate.size().1) as f64;
            f64::min(w, h)
        } else {
            1f64
        }
    };

    let resize_trigger = use_force_update();
    let onresize = Closure::<dyn FnMut(_)>::new(move |_: Event| {
        resize_trigger.force_update();
    });
    web_sys::window()
        .unwrap()
        .set_onresize(Some(onresize.as_ref().unchecked_ref()));
    onresize.forget(); // Magic!

    html! {
        <div class="plate_container">
            if let Some(spi) = props.source_dims.clone() {
            if let Some(dpi) = props.destination_dims.clone() {
            <div class="plate_container--source">
                <h2>{spi.name.clone()}</h2>
                <SourcePlate source_plate={spi.clone()} destination_plate={dpi.clone()}
                cell_height={cell_height}/>
            </div>
            <div class="plate_container--destination">
                <h2>{dpi.name.clone()}</h2>
                <DestinationPlate source_plate={spi.clone()} destination_plate={dpi.clone()}
                cell_height={cell_height}/>
            </div>
            } else {
                <h2>{"No Destination Plate Selected"}</h2>
            }
            } else {
                <h2>{"No Source Plate Selected"}</h2>
            }
        </div>
    }
}
