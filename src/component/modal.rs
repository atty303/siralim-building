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
        let class = if show {
            "body body--modal-activated"
        } else {
            "body"
        };
        let window = web_sys::window().unwrap();
        let document: web_sys::Document = window.document().unwrap();
        let body = document.body().unwrap();
        body.set_class_name(class);
    });
    cx.render(rsx!(
        div { class: if *cx.props.show { "modal--visible" } else { "modal--hidden" },
            div { class: "modal__inner",
                div { class: "modal__title",
                    div { class: "modal__text" }
                    div { class: "modal__close", button { onclick: move |e| cx.props.on_request_close.call(()) } }
                }
            }
            div { class: "modal__content", &cx.props.children }
        }
    ))
}

pub fn use_modal(cx: &ScopeState) {}
