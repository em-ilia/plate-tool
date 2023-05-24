#![allow(non_snake_case)]
use yew::prelude::*;

use crate::data::plate_instances::PlateInstance;

use super::source_plate::SourcePlate;
use super::destination_plate::DestinationPlate;

#[derive(Properties, PartialEq)]
pub struct PlateContainerProps {
    pub source_dims: Option<PlateInstance>,
    pub destination_dims: Option<PlateInstance>,
}

#[function_component]
pub fn PlateContainer(props: &PlateContainerProps) -> Html {
    html! {
        <div class="plate_container">
            if let Some(spi) = props.source_dims.clone() {
            <div>
                <h2>{spi.name.clone()}</h2>
                <SourcePlate plate={spi} />
            </div>
            } else {
                <h2>{"No Source Plate Selected"}</h2>
            }
            if let Some(dpi) = props.destination_dims.clone() {
            <div>
                <h2>{dpi.name.clone()}</h2>
                <DestinationPlate plate={dpi} />
            </div>
            } else {
                <h2>{"No Destination Plate Selected"}</h2>
            }
        </div>
    }
}
