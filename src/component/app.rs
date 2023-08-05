#![allow(non_snake_case)]

use dioxus::prelude::*;
use fermi::{use_atom_state, use_init_atom_root};

use crate::atom;
use crate::component::footer::Footer;
use crate::component::modal::use_modal;
use crate::component::navbar::NavBar;
use crate::component::party_member::PartyMember;
use crate::component::traits_table::TraitsModal;
use crate::embed_data::TRAITS_MAP;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let trait_modal = use_modal(cx);

    let party = use_atom_state(cx, &atom::PARTY);

    render! {
        NavBar {}

        h2 {
            class: "text-xl text-center text-secondary my-4",
            "PARTY"
        }

        div {
            class: "mx-4 space-y-4",
            for (i, m) in party.get().iter().enumerate() {
                PartyMember {
                    member: m.clone(),
                    on_trait_click: move |trait_index: usize| {
                        let p = party.clone();
                        trait_modal.show_modal(move |e| {
                            p.with_mut(|p| p[i].traits[trait_index] = Some(&TRAITS_MAP[&e]));
                        });
                    }
                }
            }
        }

        Footer {}

        trait_modal.component(cx, TraitsModal)
    }

    // cx.render(rsx! {
    //     div {
    //         DndState {
    //             Draggable {
    //                 DragHandle {
    //                     div { "Drag me" }
    //                 }
    //                 div { "outer" }
    //             }
    //             Droppable {
    //                 div { "Drop here" }
    //             }
    //         }
    //     }
    // })
}
