use crate::component::app::App;

mod atom;
mod component;
mod embed_data;
mod hooks;
mod state;
mod url_save;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus_web::launch(App);
}
