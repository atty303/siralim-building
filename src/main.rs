//use crate::components::app::{App, AppProps};

// mod components;
// mod save;
// mod state;

mod atom;
mod component;
mod embed_data;

use crate::component::app::App;
use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(App);

    // let props = AppProps {};
    //
    // yew::Renderer::<App>::with_props(props).render();
}
