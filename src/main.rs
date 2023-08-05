// mod save;

use crate::component::app::App;

mod atom;
mod component;
mod embed_data;
mod state;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(App);

    // let props = AppProps {};
}
