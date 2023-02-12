#![allow(non_snake_case)]

mod creatures;
mod data;

use crate::creatures::CreatureTable;
use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let creatures = data::Data::creatures();
    cx.render(rsx! {
        div {
            input { }
            div {
                CreatureTable { items: creatures }
            }
        }
    })
}
