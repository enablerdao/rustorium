mod api;
mod components;
mod models;
mod pages;
mod utils;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::components::layout::Layout;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Rustorium Web UI starting...");
    
    yew::Renderer::<App>::new().render();
    Ok(())
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Layout />
    }
}