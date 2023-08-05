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
use crate::url_save;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let trait_modal = use_modal(cx);

    let url_state = use_atom_state(cx, &atom::URL_STATE);

    use_effect(cx, url_state.get(), move |state| {
        to_owned![state];
        async move {
            url_save::set_to_url(&state);
        }
    });

    render! {
        NavBar {}

        h2 {
            class: "text-xl text-center text-secondary my-4",
            "PARTY"
        }

        div {
            class: "mx-4 space-y-4",
            for (i, m) in url_state.get().party.iter().enumerate() {
                PartyMember {
                    member: m.clone(),
                    on_trait_click: move |trait_index| {
                        let us = url_state.clone();
                        trait_modal.show_modal(move |e| {
                            us.with_mut(|us| {
                                us.party[i].traits[trait_index] = Some(&TRAITS_MAP[&e]);
                            });
                        });
                    },
                    on_trait_clear: move |trait_index| {
                        let us = url_state.clone();
                        us.with_mut(|us| {
                            us.party[i].traits[trait_index] = None;
                        });
                    },
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
