use data::Data;
use defy::defy;
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

    defy! {
        for t in props.value.iter() {
            if let Some(w) = data.keywords.get(t) {
                span(class = format!("desc-keyword {}", w.category)) {
                    img(class = "icon", src = format!("image/{}", w.icon));
                    + t.as_str();
                }
            } else {
                if let Some(e) = data.effects.get(t) {
                    span(class = format!("tooltip desc-effect {}", e.category)) {
                        span(class = "tooltip-text") {
                            + e.description.clone();
                        }
                        img(class = "icon", src = format!("status_icons/{}", e.icon));
                        + t.as_str();
                    }
                } else {
                    if let Ok(s) = data.search_spell(format!("name:\"{}\"", t).as_str()) {
                        if let Some(x) = s.get(0) {
                            span(class = "tooltip desc-spell") {
                                span(class = "tooltip-text") {
                                    + x.description.clone();
                                }
                                + t.as_str();
                            }
                        } else {
                            span {
                                + t.as_str();
                            }
                        }
                    } else {
                        span {
                            + t.as_str();
                        }
                    }
                }
            }
        }
    }
}
