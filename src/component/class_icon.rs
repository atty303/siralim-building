#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn ClassIcon<'a>(cx: Scope<'a>, name: &'a str, class: Option<&'a str>) -> Element<'a> {
    let lower_name = name.to_lowercase();
    if ["nature", "chaos", "sorcery", "death", "life"].contains(&lower_name.as_str()) {
        render! {
            img {
                class: "inline-block {class.unwrap_or_default()}",
                src: "images/{lower_name}.png",
            }
        }
    } else {
        None
    }
}
