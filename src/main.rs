#![allow(non_snake_case)]

mod components;
mod creatures;
mod data;
mod party;

use crate::creatures::CreatureModal;
use crate::data::Creature;
use crate::party::PartyMember;
use dioxus::prelude::*;
use dioxus_web::Config;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_web::launch_with_props(
        App,
        AppProps {
            creatures: data::Data::creatures(),
        },
        Config::default(),
    );
}

#[derive(PartialEq, Props)]
struct AppProps {
    creatures: Vec<Creature>,
}

fn App(cx: Scope<AppProps>) -> Element {
    let show_creatures_modal = use_state(cx, || false);

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
                    creature: (Some(cx.props.creatures.get(0).unwrap().clone()), None, None)
                }
            }
        }
        CreatureModal {
            items: &cx.props.creatures,
            show: **show_creatures_modal,
            on_select: move |_| {
                show_creatures_modal.set(false);
            }
        }
    })
}
