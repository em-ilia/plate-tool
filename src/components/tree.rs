#![allow(non_snake_case)]

use yew::prelude::*;
use yewdux::prelude::*;

use crate::data::{plate_instances::PlateInstance, transfer::Transfer};
use crate::data::plate::*;
use crate::components::states::MainState;

#[derive(PartialEq, Properties)]
pub struct TreeProps {
    pub open_new_plate_callback: Callback<()>,
}

#[function_component]
pub fn Tree(props: &TreeProps) -> Html {
    let (state, dispatch) = use_store::<MainState>();
    let source_plates = state.source_plates.iter()
        .map(|spi| {
            html!{ <li> {String::from(spi)} </li> }
        }).collect::<Html>();
    let dest_plates = state.destination_plates.iter()
        .map(|spi| {
            html!{ <li> {String::from(spi)} </li> }
        }).collect::<Html>();


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
            </ul>
            </div>
            // Temporary
            <div>
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
