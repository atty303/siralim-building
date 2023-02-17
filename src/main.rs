use crate::components::app::{App, AppProps};

mod components;
mod embed_data;
mod embed_directory;
mod save;
mod state;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let props = AppProps {
        data: embed_data::load(),
    };

    yew::Renderer::<App>::with_props(props).render();
}
