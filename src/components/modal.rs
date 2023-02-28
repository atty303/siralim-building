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

    let mut modal_classes = Classes::from("modal-overlay");
    modal_classes.push(if props.show { "visible" } else { "hidden" });
    html! {
        <div class={modal_classes}>
            <div class={class}>
                <div class="title">
                    <div class="text"></div>
                    <div class="close">
                        <button onclick={onclick}><Icon icon_id={IconId::BootstrapXLg} /></button>
                    </div>
                </div>
                <div class="content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
