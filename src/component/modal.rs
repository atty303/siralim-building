#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::component::outline_icon::OutlineIcon;

#[derive(Props)]
pub struct ModalProps<'a> {
    show: &'a bool,
    on_request_close: EventHandler<'a, ()>,
    children: Element<'a>,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element {
    // use_effect(cx, (&cx.props.show.clone(),), |(show,)| async move {
    //     let class = if show { "overflow-hidden" } else { "" };
    //     let window = web_sys::window().unwrap();
    //     let document: web_sys::Document = window.document().unwrap();
    //     let body = document.body().unwrap();
    //     body.set_class_name(class);
    // });
    render! {
        dialog { class: if *cx.props.show { "modal modal-open" } else { "modal" },
            form {
                method: "dialog",
                class: "modal-box max-w-full h-full relative",
                style: "width: calc(100vw - 5em)",
                button {
                    class: "btn btn-sm btn-circle btn-ghost absolute right-1 top-1",
                    OutlineIcon {
                        icon: Shape::XMark,
                    }
                }
                &cx.props.children
            }
        }
    }
}

pub fn use_modal(cx: &ScopeState) {}
