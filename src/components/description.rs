use data::Data;
use implicit_clone::unsync::IString;
use std::ops::Deref;
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
                        <span class={format!("effect {}", e.category)}>{t.as_str()}</span>
                    }
                } else {
                    html! { <span>{t.as_str()}</span> }
                }
            }).collect::<Vec<Html>>()}
        </>
    }
}
