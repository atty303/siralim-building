use yew::prelude::*;

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
                    <button onclick={onclick}>{"close"}</button>
                </div>
                <div class="content">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
