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
                            <img class="icon" src={format!("status_icons/{}", e.icon)} />
                            {t.as_str()}
                        </span>
                    }
                } else if let Ok(s) = data.search_spell(format!("name:\"{}\"", t).as_str()) {
                    if let Some(x) = s.get(0) {
                        html! {
                            <span class={"tooltip spell"}>
                                <span class="tooltip-text">
                                    {x.description.clone()}
                                </span>
                                {t.as_str()}
                            </span>
                        }
                    } else {
                        html! { <span>{t.as_str()}</span> }
                    }
                } else {
                    html! { <span>{t.as_str()}</span> }
                }
            }).collect::<Vec<Html>>()}
        </>
    }
}
