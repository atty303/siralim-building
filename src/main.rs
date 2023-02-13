#![allow(non_snake_case)]

mod components;
mod creatures;
mod data;
mod party;

use crate::creatures::CreatureModal;
use crate::data::Creature;
use crate::party::{Member, Party, PartyMemberSwapEvent};
use dioxus::prelude::*;
use dioxus_web::Config;
use std::borrow::BorrowMut;

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
    let party = use_state(cx, || {
        vec![
            Member {
                primary_creature: Some(cx.props.creatures.get(0).unwrap().clone()),
                fused_creature: None,
                artifact_creature: None,
            },
            Member {
                primary_creature: Some(cx.props.creatures.get(100).unwrap().clone()),
                fused_creature: None,
                artifact_creature: None,
            },
        ]
    });

    cx.render(rsx! {
        div {
            "header"
            button {
                onclick: move |_| show_creatures_modal.set(true),
                "open"
            }
            Party {
                party: party
                on_swap: move |e: PartyMemberSwapEvent| {
                    log::info!("{:?}", e);
                    let from_member = party.get().get(e.from_position as usize).unwrap().clone();
                    let from = from_member.get_creature(e.from_index);
                    let to_member = party.get().get(e.to_position as usize).unwrap().clone();
                    let to = to_member.get_creature(e.to_index);

                    let mut f = from_member.clone();
                    f.set_creature(e.from_index, &to);
                    let mut t = to_member.clone();
                    t.set_creature(e.to_index, &from);

                    let mut p = party.get().to_vec();
                    p[e.from_position as usize] = t;
                    p[e.to_position as usize] = f;

                    party.set(p);
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
