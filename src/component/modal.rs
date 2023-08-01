#![allow(non_snake_case)]

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

pub fn use_modal(cx: &ScopeState) -> &mut UseModal {
    let modalRef: &UseRef<Option<web_sys::HtmlDialogElement>> = use_ref(cx, || None);

    cx.use_hook(move || UseModal {
        modalRef: modalRef.clone(),
    })
}

#[derive(Clone)]
pub struct UseModal {
    pub modalRef: UseRef<Option<web_sys::HtmlDialogElement>>,
}

#[derive(Props, Clone)]
pub struct ModalProps<'a> {
    pub children: Element<'a>,
}

impl UseModal {
    pub fn Modal<'a>(&'a mut self, cx: &'a ScopeState, children: Element<'a>) -> Element<'a> {
        render! {
            dialog {
                class: "modal",
                onmounted: |e| {
                    log::debug!("{:?}", e.get_raw_element().unwrap().downcast_ref::<web_sys::Element>());
                    let el = e
                    .get_raw_element()
                    .expect("expecting raw element")
                    .downcast_ref::<web_sys::Element>()
                    .expect("expecting Element")
                    .dyn_ref::<web_sys::HtmlDialogElement>()
                    .expect("expecting HtmlDialogElement");
                    log::debug!("{:?}", el);

                    self.modalRef.write().replace(el.clone());
                },
                form {
                    method: "dialog",
                    class: "modal-box max-w-full h-full relative",
                    style: "width: calc(100vw - 5em)",
                    button {
                        class: "btn btn-sm btn-circle btn-ghost absolute right-1 top-1",
                        onclick: |_| {
                            self.close();
                        },
                        OutlineIcon {
                            icon: Shape::XMark,
                        }
                    }
                    &children
                }
            }
        }
    }

    pub fn show(&self) {
        if let Some(el) = self.modalRef.read().as_ref() {
            el.show();
        };
    }

    pub fn close(&self) {
        if let Some(el) = self.modalRef.read().as_ref() {
            el.close();
        };
    }
}
