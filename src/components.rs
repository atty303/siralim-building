#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Props)]
pub struct ModalProps<'a> {
    show: bool,
    on_request_close: EventHandler<'a, ()>,
    children: Element<'a>,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element {
    if cx.props.show {
        cx.render(rsx! {
            div {
                class: "modal-overlay",
                div {
                    class: "modal",
                    div {
                        class: "title",
                        button { onclick: move |_|  cx.props.on_request_close.call(()), "close" }
                    }
                    div {
                        class: "content",
                        &cx.props.children
                    }
                }
            }
        })
    } else {
        None
    }
}

#[derive(PartialEq, Props)]
pub struct ClassIconProps<'a> {
    value: &'a String,
}

pub fn ClassIcon<'a>(cx: Scope<'a, ClassIconProps<'a>>) -> Element {
    let lower_class = cx.props.value.to_lowercase();
    let icon = if ["nature", "chaos", "sorcery", "death", "life"].contains(&lower_class.as_str()) {
        rsx! { img { src: format_args!("image/{}.png", &lower_class) } }
    } else {
        rsx! { "" }
    };
    cx.render(rsx! { icon })
}
