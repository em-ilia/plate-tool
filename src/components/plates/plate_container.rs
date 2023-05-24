#![allow(non_snake_case)]
use yew::prelude::*;
use yewdux::prelude::*;
use super::super::states::MainState;

use super::source_plate::SourcePlate;
use super::destination_plate::DestinationPlate;

#[derive(Properties, PartialEq)]
pub struct PlateContainerProps {
    pub source_dims: (u8,u8),
    pub destination_dims: (u8,u8)
}

#[function_component]
pub fn PlateContainer(props: &PlateContainerProps) -> Html {
    let (state, dispatch) = use_store::<MainState>();

    html! {
        <div class="plate_container">
            <SourcePlate width={props.source_dims.0} height={props.source_dims.1} />
            <DestinationPlate width={props.destination_dims.0} height={props.destination_dims.1} />
        </div>
    }
}
