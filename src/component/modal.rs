#![allow(non_snake_case)]

use dioxus::core::{DynamicNode, IntoDynNode};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use wasm_bindgen::JsCast;

use crate::component::outline_icon::OutlineIcon;

pub fn use_modal(cx: &ScopeState) -> &ModalState {
    let modalRef: &UseRef<Option<web_sys::HtmlDialogElement>> = use_ref(cx, || None);

    cx.use_hook(move || ModalState {
        modalRef: modalRef.clone(),
        component: |cx| render! {
            dialog {
                class: "modal backdrop:backdrop-blur",
                onmounted: move |e| {
                    log::debug!("{:?}", e.get_raw_element().unwrap().downcast_ref::<web_sys::Element>());
                    let el = e
                        .get_raw_element().expect("expecting raw element")
                        .downcast_ref::<web_sys::Element>().expect("expecting Element")
                        .dyn_ref::<web_sys::HtmlDialogElement>().expect("expecting HtmlDialogElement");

                    cx.props.modalRef.write().replace(el.clone());
                },
                div {
                    class: "modal-box w-[calc(100vw-5em)] max-w-full h-full relative",
                    button {
                        class: "btn btn-sm btn-circle btn-ghost absolute right-1 top-1",
                        tabindex: "-1",
                        onclick: move |_| {
                            if let Some(el) = cx.props.modalRef.read().as_ref() {
                                el.close();
                            };
                        },
                        OutlineIcon {
                            icon: Shape::XMark,
                        }
                    }
                    &cx.props.children
                }
                form {
                    class: "modal-backdrop",
                    method: "dialog",
                    button { "close" }
                }
            }
        },
    })
}

pub struct ModalState {
    pub modalRef: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub component: for<'a> fn(Scope<'a, ModalProps<'a>>) -> Element<'a>,
}

#[derive(Props, Clone)]
pub struct ModalProps<'a> {
    pub modalRef: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub children: Element<'a>,
}

impl ModalState {
    pub fn component<'a>(&self, cx: &'a ScopeState, children: Element<'a>) -> DynamicNode<'a> {
        cx.component(
            self.component,
            ModalProps {
                modalRef: self.modalRef.clone(),
                children: children.clone(),
            },
            "Modal",
        )
    }

    pub fn show(&self) {
        if let Some(el) = self.modalRef.read().as_ref() {
            el.show_modal();
        };
    }

    pub fn close(&self) {
        if let Some(el) = self.modalRef.read().as_ref() {
            el.close();
        };
    }
}
