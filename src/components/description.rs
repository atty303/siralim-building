use std::rc::Rc;

use implicit_clone::unsync::IString;
use yew::prelude::*;

use data::Data;

use crate::components::tooltip::Tooltip;

#[derive(Properties, PartialEq)]
pub struct DescriptionProps {
    pub value: Vec<IString>,
}

#[function_component(Description)]
pub fn description(props: &DescriptionProps) -> Html {
    let data = use_context::<Rc<Data>>().unwrap();

    html! {
        <span class="desc">
            {props.value.iter().map(|t| {
                if let Some(w) = data.keywords.get(t) {
                    html! {
                        <span class={format!("desc__keyword desc__keyword-{}", w.category)}>
                            <img class="desc__icon" src={format!("image/{}", w.icon)} />
                            {t.as_str()}
                        </span>
                    }
                } else if let Some(e) = data.effects.get(t) {
                    let tooltip = html! {
                        {e.description.clone()}
                    };
                    html! {
                        <Tooltip {tooltip}>
                            <span class={format!("desc__effect desc__effect-{}", e.category)}>
                                <img class="desc__icon" src={format!("status_icons/{}", e.icon)} />
                                {t.as_str()}
                            </span>
                        </Tooltip>
                    }
                } else if let Ok(s) = data.search_spell(format!("name:\"{}\"", t).as_str()) {
                    if let Some(x) = s.get(0) {
                        let tooltip = html! {
                            // TODO: nested tooltip ?
                            <span>{x.description.clone()}</span>
                        };
                        html! {
                            <Tooltip {tooltip}>
                                <span class="desc__spell">
                                    {t.as_str()}
                                </span>
                            </Tooltip>
                        }
                    } else {
                        html! { <span>{t.as_str()}</span> }
                    }
                } else {
                    html! { <span>{t.as_str()}</span> }
                }
            }).collect::<Vec<Html>>()}
        </span>
    }
}
