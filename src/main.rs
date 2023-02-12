#![allow(non_snake_case)]

mod components;
mod creatures;
mod data;
mod party;

use crate::creatures::CreatureModal;
use crate::party::PartyMember;
use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let creatures = data::Data::creatures();

    let show_creatures_modal = use_state(cx, || true);

    cx.render(rsx! {
        div {
            "header"
            button {
                onclick: move |_| show_creatures_modal.set(true),
                "open"
            }
            div {
                class: "party",
                PartyMember {
                    creature: (Some(creatures.get(0).unwrap().clone()), None, None)
                }
            }
        }
        CreatureModal {
            items: creatures,
            show: **show_creatures_modal,
            on_select: move |_| {
                show_creatures_modal.set(false);
            }
        }
    })
}
