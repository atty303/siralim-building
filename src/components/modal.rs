use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub show: bool,
    pub class: Classes,
    pub on_request_close: Callback<()>,
    pub children: Children,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let onclick = {
        let on_request_close = props.on_request_close.clone();
        Callback::from(move |_| on_request_close.emit(()))
    };

    let mut class = props.class.clone();
    class.push("modal");

    let modal_class = if props.show {
        "modal-overlay--visible"
    } else {
        "modal-overlay--hidden"
    };
    html! {
        <div class={modal_class}>
            <div class={class}>
                <div class="modal__title">
                    <div class="modal__text"></div>
                    <div class="modal__close">
                        <button onclick={onclick}><Icon icon_id={IconId::BootstrapXLg} /></button>
                    </div>
                </div>
                <div class="modal__content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
