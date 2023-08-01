#![allow(non_snake_case)]

use dioxus::core::{DynamicNode, IntoDynNode};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use wasm_bindgen::JsCast;

use crate::component::outline_icon::OutlineIcon;

//#[inline_props]
// pub fn Modal<'a>(cx: Scope<'a>, modal: &'a mut UseModal, children: Element<'a>) -> Element<'a> {
//     render! {
//         dialog {
//             class: "modal",
//             onmounted: move |e| {
//                 modal.deref_mut().onmount(e);
//             },
//             form {
//                 method: "dialog",
//                 class: "modal-box max-w-full h-full relative",
//                 style: "width: calc(100vw - 5em)",
//                 button {
//                     class: "btn btn-sm btn-circle btn-ghost absolute right-1 top-1",
//                     onclick: move |_| {
//                         modal.close();
//                     },
//                     OutlineIcon {
//                         icon: Shape::XMark,
//                     }
//                 }
//                 &cx.props.children
//             }
//         }
//     }
// }

pub fn use_modal(cx: &ScopeState) -> &ModalState {
    let modalRef: &UseRef<Option<web_sys::HtmlDialogElement>> = use_ref(cx, || None);

    cx.use_hook(move || ModalState {
        modalRef: modalRef.clone(),
        component: |cx| render! {
            dialog {
                class: "modal modal-open",
                onmounted: move |e| {
                    log::debug!("{:?}", e.get_raw_element().unwrap().downcast_ref::<web_sys::Element>());
                    let el = e
                    .get_raw_element()
                    .expect("expecting raw element")
                    .downcast_ref::<web_sys::Element>()
                    .expect("expecting Element")
                    .dyn_ref::<web_sys::HtmlDialogElement>()
                    .expect("expecting HtmlDialogElement");
                    log::debug!("{:?}", el);

                    cx.props.modalRef.write().replace(el.clone());
                },
                form {
                    method: "dialog",
                    class: "modal-box max-w-full h-full relative",
                    style: "width: calc(100vw - 5em)",
                    button {
                        class: "btn btn-sm btn-circle btn-ghost absolute right-1 top-1",
                        onclick: move |_| {
                            // self.close();
                        },
                        OutlineIcon {
                            icon: Shape::XMark,
                        }
                    }
                    &cx.props.children
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

    //     pub fn modal<'a>(&self, _cx: &'a ScopeState) -> fn(Scope<'a, ModalProps<'a>>) -> Element<'a> {
    //         move |cx| {
    //             render! {
    //                 dialog {
    //                     class: "modal modal-open",
    //                     onmounted: move |e| {
    //                         log::debug!("{:?}", e.get_raw_element().unwrap().downcast_ref::<web_sys::Element>());
    //                         let el = e
    //                         .get_raw_element()
    //                         .expect("expecting raw element")
    //                         .downcast_ref::<web_sys::Element>()
    //                         .expect("expecting Element")
    //                         .dyn_ref::<web_sys::HtmlDialogElement>()
    //                         .expect("expecting HtmlDialogElement");
    //                         log::debug!("{:?}", el);
    //
    //                         // cx.props.modalRef.write().replace(el.clone());
    //                         // self.modalRef.write().replace(el.clone());
    //                     },
    //                     form {
    //                         method: "dialog",
    //                         class: "modal-box max-w-full h-full relative",
    //                         style: "width: calc(100vw - 5em)",
    //                         button {
    //                             class: "btn btn-sm btn-circle btn-ghost absolute right-1 top-1",
    //                             onclick: move |_| {
    //                                 // self.close();
    //                             },
    //                             OutlineIcon {
    //                                 icon: Shape::XMark,
    //                             }
    //                         }
    //                         &cx.props.children
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     pub fn show(&self) {
    //         if let Some(el) = self.modalRef.read().as_ref() {
    //             el.show();
    //         };
    //     }
    //
    //     pub fn close(&self) {
    //         if let Some(el) = self.modalRef.read().as_ref() {
    //             el.close();
    //         };
    //     }
}
