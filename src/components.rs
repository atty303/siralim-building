#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct ClassIconProps<'a> {
    value: &'a String,
}

pub fn ClassIcon<'a>(cx: Scope<'a, ClassIconProps<'a>>) -> Element {
    let lower_class = cx.props.value.to_lowercase();
    let icon = if ["nature", "chaos", "sorcery", "death", "life"].contains(&lower_class.as_str()) {
        rsx! { img { src: format_args!("image/{}.png", &lower_class) } }
    } else {
        rsx! { "" }
    };
    cx.render(rsx! { icon })
}
