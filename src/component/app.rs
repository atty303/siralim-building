#![allow(non_snake_case)]

use data::spell::Spell;
use dioxus::prelude::*;
use fermi::{use_init_atom_root, AtomRoot};

use crate::component::drag::{DndState, DragHandle, Draggable, Droppable};
use crate::component::footer::Footer;
use crate::component::modal::{use_modal, ModalProps};
use crate::component::navbar::NavBar;
use crate::component::party_member::PartyMember;
use crate::component::party_trait::PartyTrait;
use crate::component::traits_table::TraitsModal;

pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    let trait_modal_state = use_modal(cx);
    let TraitModal = trait_modal_state.component(cx, render! { TraitsModal {} });

    let spell_modal_state = use_modal(cx);
    let SpellModal = spell_modal_state.component(cx, render! { "Spells" });

    render! {
        NavBar {}

        h2 {
            class: "text-xl text-center text-secondary my-4",
            onclick: move |_| trait_modal_state.show(),
            "PARTY"
        }
        h2 {
            class: "text-xl text-center text-secondary my-4",
            onclick: move |_| spell_modal_state.show(),
            "PARTY"
        }

        div {
            class: "mx-4 space-y-4",
            PartyMember {}
            PartyMember {}
            PartyMember {}
            PartyMember {}
        }

        Footer {}

        TraitModal
        SpellModal
    }

    // cx.render(rsx! {
    //     div {
    //         draggable: false,
    //         ondragstart: move |e| println!("{e:?}"),
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
