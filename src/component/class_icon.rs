#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn ClassIcon<'a>(cx: Scope<'a>, class: &'a str) -> Element<'a> {
    let lower_class = class.to_lowercase();
    if ["nature", "chaos", "sorcery", "death", "life"].contains(&lower_class.as_str()) {
        render! {
            img {
                class: "inline-block",
                src: "images/{lower_class}.png",
            }
        }
    } else {
        None
    }
}
