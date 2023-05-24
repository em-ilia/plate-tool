#![allow(non_snake_case)]
use yew::prelude::*;

use super::source_plate::SourcePlate;
use super::destination_plate::DestinationPlate;

#[derive(Properties, PartialEq)]
pub struct PlateContainerProps {
    pub source_dims: Option<(u8,u8)>,
    pub destination_dims: Option<(u8,u8)>,
}

#[function_component]
pub fn PlateContainer(props: &PlateContainerProps) -> Html {
    html! {
        <div class="plate_container">
            if let Some((w,h)) = props.source_dims {
            <SourcePlate width={w} height={h} />
            } else {
                <h2>{"No Source Plate Selected"}</h2>
            }
            if let Some((w,h)) = props.destination_dims {
            <DestinationPlate width={w} height={h} />
            } else {
                <h2>{"No Destination Plate Selected"}</h2>
            }
        </div>
    }
}
