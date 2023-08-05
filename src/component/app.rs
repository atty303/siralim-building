#![allow(non_snake_case)]

use dioxus::prelude::*;
use fermi::{use_init_atom_root, use_read};

use crate::atom;
use crate::component::footer::Footer;
use crate::component::modal::use_modal;
use crate::component::navbar::NavBar;
use crate::component::party_member::PartyMember;
use crate::component::traits_table::TraitsModal;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let trait_modal_state = use_modal(cx);
    let TraitModal = trait_modal_state.component(cx, TraitsModal);

    // let spell_modal_state = use_modal(cx);
    // let SpellModal = spell_modal_state.component(cx, render! { "Spells" });

    let party = use_read(cx, &atom::PARTY);

    render! {
        NavBar {}

        h2 {
            class: "text-xl text-center text-secondary my-4",
            "PARTY"}

        div {
            class: "mx-4 space-y-4",
            for m in party {
                PartyMember {
                    member: m.clone(),
                    on_trait_click: |trait_index: usize| {
                        // trait_modal_state.show();
                        trait_modal_state.show_modal(move |e| {
                            log::debug!("trait_index: {}, {:?}", trait_index, e);
                        });
                    }
                }
            }
        }

        Footer {}

        TraitModal
        // SpellModal
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
