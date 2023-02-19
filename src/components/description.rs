use implicit_clone::unsync::IString;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DescriptionProps {
    pub value: Vec<IString>,
}

#[function_component(Description)]
pub fn description(props: &DescriptionProps) -> Html {
    html! {
        <>
            {props.value.iter().map(|t| html! {
                <span>{t.as_str()}</span>
            }).collect::<Vec<Html>>()}
        </>
    }
}
