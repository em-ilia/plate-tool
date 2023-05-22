#![allow(non_snake_case)]
use yew::prelude::*;
use super::plates::plate_container::PlateContainer;
use super::tree::Tree;
use super::transfer_menu::TransferMenu;


#[function_component]
pub fn MainWindow() -> Html {
    html!{
        <div class="main_container">
            <Tree />
            <TransferMenu />
            <PlateContainer source_dims={(24,16)} destination_dims={(24,16)}/>
        </div>
    }
}
