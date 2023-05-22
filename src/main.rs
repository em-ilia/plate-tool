#[cfg(debug_assertions)]
use plate_tool::plate_test;

use plate_tool::App;
use wasm_logger;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    //plate_test();
}
