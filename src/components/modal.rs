use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub show: bool,
    pub on_request_close: Callback<()>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    use_effect_with_deps(
        move |v| {
            let class = if *v {
                "body body--modal-activated"
            } else {
                "body"
            };
            let window = web_sys::window().unwrap();
            let document: web_sys::Document = window.document().unwrap();
            let body = document.body().unwrap();
            body.set_class_name(class);
        },
        props.show.clone(),
    );

    let onclick = {
        let on_request_close = props.on_request_close.clone();
        Callback::from(move |_| on_request_close.emit(()))
    };

    let modal_class = if props.show {
        "modal--visible"
    } else {
        "modal--hidden"
    };
    html! {
        <div class={modal_class}>
            <div class="modal__inner">
                <div class="modal__title">
                    <div class="modal__text"></div>
                    <div class="modal__close">
                        <button {onclick}><Icon icon_id={IconId::BootstrapXLg} /></button>
                    </div>
                </div>
                <div class="modal__content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
