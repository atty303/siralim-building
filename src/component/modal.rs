#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct ModalProps<'a> {
    show: &'a bool,
    on_request_close: EventHandler<'a, ()>,
    children: Element<'a>,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element {
    use_effect(cx, (&cx.props.show.clone(),), |(show,)| async move {
        let class = if show { "overflow-hidden" } else { "" };
        let window = web_sys::window().unwrap();
        let document: web_sys::Document = window.document().unwrap();
        let body = document.body().unwrap();
        body.set_class_name(class);
    });
    cx.render(rsx!(
        div { class: if *cx.props.show { "fixed inset-0 w-full h-full bg-black visible" } else { "fixed inset-0 w-full h-full bg-black invisible" },
            div { class: "absolute inset-8 bg-white shadow z-10",
                div { class: "w-full bg-neutral text-neutral-content h-8 text-right flex border-b",
                    div { class: "grow" }
                    div { class: "modal__close", button { onclick: move |e| cx.props.on_request_close.call(()) } }
                }
                div { class: "p-4", &cx.props.children }
            }
        }
    ))
}

pub fn use_modal(cx: &ScopeState) {}
