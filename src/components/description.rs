use data::Data;
use implicit_clone::unsync::IString;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DescriptionProps {
    pub value: Vec<IString>,
}

#[function_component(Description)]
pub fn description(props: &DescriptionProps) -> Html {
    let data = use_context::<Rc<Data>>().unwrap();

    html! {
        <>
            {props.value.iter().map(|t| {
                if let Some(e) = data.effects.get(t) {
                    html! {
                        <span class={format!("tooltip effect {}", e.category)}>
                            <span class="tooltip-text">
                                {e.description.clone()}
                            </span>
                            {t.as_str()}
                        </span>
                    }
                } else {
                    html! { <span>{t.as_str()}</span> }
                }
            }).collect::<Vec<Html>>()}
        </>
    }
}
