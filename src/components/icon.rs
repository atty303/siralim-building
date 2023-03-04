use defy::defy;
use implicit_clone::unsync::IString;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ClassIconProps {
    pub value: IString,
}

#[function_component(ClassIcon)]
pub fn class_icon(props: &ClassIconProps) -> Html {
    let lower_class = props.value.to_lowercase();
    if ["nature", "chaos", "sorcery", "death", "life"].contains(&lower_class.as_str()) {
        defy! { img(src = format!("image/{}.png", &lower_class)); }
    } else {
        defy! {}
    }
}
