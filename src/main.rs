//use crate::components::app::{App, AppProps};

// mod components;
// mod embed_data;
// mod embed_directory;
// mod save;
// mod state;

mod component;

use crate::component::app::App;
use dioxus::prelude::*;

fn main() {
    // wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(App);

    // let props = AppProps {};
    //
    // yew::Renderer::<App>::with_props(props).render();
}
