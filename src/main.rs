#![allow(non_snake_case)]

use dioxus::prelude::*;
use rust_embed::RustEmbed;
use log::{LevelFilter};

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let data = Data::get("data/creatures.json").unwrap();
    let str = std::str::from_utf8(data.data.as_ref()).unwrap();
    let creatures: Vec<Creature> = serde_json::from_slice(data.data.as_ref()).unwrap();
    cx.render(rsx! {
        div {
            creatures.iter().map(|c| rsx!( p { "{c.creature}" }))
        }
    })
}

#[derive(RustEmbed)]
#[folder = "src/data/"]
#[prefix = "data/"]
struct Data;

#[derive(serde::Deserialize)]
struct Creature {
    class: String,
    family: String,
    creature: String,
}