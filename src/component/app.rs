#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::component::drag::{DndState, DragHandle, Draggable, Droppable};

pub fn App(cx: Scope) -> Element {
    let show = use_state(cx, || false);

    cx.render(rsx! {
        div {
            class: "text-lg bg-slate-100",
            draggable: false,
            ondragstart: move |e| println!("{e:?}"),
            onclick: move |_| show.set(!(show.get())),
            // Modal { show: show, on_request_close: move |_| println!("close"), div { "Hello, world!" } },
            DndState {
                Draggable {
                    DragHandle {
                        div { "Drag me" }
                    }
                    div { "outer" }
                }
                Droppable {
                    div { "Drop here" }
                }
            }
        }
    })
}
