#![allow(non_snake_case)]

use dioxus::core::DynamicNode;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use wasm_bindgen::JsCast;

use crate::component::outline_icon::OutlineIcon;

pub fn use_modal<T: 'static>(cx: &ScopeState) -> &UseModal<T> {
    let modalRef: &UseRef<Option<web_sys::HtmlDialogElement>> = use_ref(cx, || None);
    let done = use_ref(cx, || None);

    cx.use_hook(move || UseModal {
        modalRef: modalRef.clone(),
        done: done.clone(),
        component: |cx| render! {
            dialog {
                class: "modal backdrop:backdrop-blur",
                onmounted: move |e| {
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
                        tabindex: -1,
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

pub struct UseModal<T: 'static> {
    pub modalRef: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub done: UseRef<Option<Box<dyn Fn(T)>>>,
    pub component: for<'a> fn(Scope<'a, ModalProps<'a>>) -> Element<'a>,
}

#[derive(Props, Clone)]
pub struct ModalProps<'a> {
    pub modalRef: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub children: Element<'a>,
}

#[derive(Props)]
pub struct ModalDialogProps<'a, T: 'static> {
    pub on_result: EventHandler<'a, T>,
}

impl<T> UseModal<T> {
    pub fn component<'a>(
        &self,
        cx: &'a ScopeState,
        child: fn(Scope<'a, ModalDialogProps<'a, T>>) -> Element<'a>,
    ) -> DynamicNode<'a> {
        let modalRef = self.modalRef.clone();
        let done = self.done.clone();

        let child_component = cx.component(
            child,
            ModalDialogProps {
                on_result: cx.event_handler(move |e| {
                    if let Some(d) = done.read().as_ref() {
                        d(e);
                    }
                    if let Some(el) = modalRef.read().as_ref() {
                        el.close();
                    };
                }),
            },
            "ModalDialog",
        );
        cx.component(
            self.component,
            ModalProps {
                modalRef: self.modalRef.clone(),
                children: render! { child_component },
            },
            "Modal",
        )
    }

    pub fn show_modal(&self, done: impl Fn(T) + 'static) {
        *self.done.write() = Some(Box::new(done));

        if let Some(el) = self.modalRef.read().as_ref() {
            el.show_modal().expect("show_modal failed");
        };
    }

    // pub fn close(&self) {
    //     if let Some(el) = self.modalRef.read().as_ref() {
    //         el.close();
    //     };
    // }
}
