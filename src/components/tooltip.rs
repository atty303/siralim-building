use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub tooltip: Html,
    pub children: Children,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let visible = use_state(|| false);

    let onmouseenter = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(true))
    };
    let onmouseleave = {
        let visible = visible.clone();
        Callback::from(move |_| visible.set(false))
    };

    html! {
        <span class="tooltip" {onmouseenter} {onmouseleave}>
            {if *visible {
                html! { <div class="tooltip__body">{props.tooltip.clone()}</div> }
            } else {
                html! {}
            }}
            {props.children.clone()}
        </span>
    }
}
