#![allow(non_snake_case)]

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

    let traits_modal = use_modal(cx, render! { TraitsModal {} });
    // let Modal2 = cx.component(
    //     traits_modal.modal(cx),
    //     ModalProps {
    //         children: render! { TraitsModal {} },
    //     },
    //     "Modal2",
    // );

    render! {
            NavBar {}

            h2 {
                class: "text-xl text-center text-secondary my-4",
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

    //        Modal2
        }

    // cx.render(rsx! {
    //     div {
    //         draggable: false,
    //         ondragstart: move |e| println!("{e:?}"),
    //         onclick: move |_| show.set(!(show.get())),
    //         // Modal { show: show, on_request_close: move |_| println!("close"), div { "Hello, world!" } },
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
