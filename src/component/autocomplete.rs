#![allow(non_snake_case)]

use classes::classes;
use dioxus::prelude::*;

#[inline_props]
pub fn Autocomplete<'a>(
    cx: Scope,
    class: &'a str,
    value: &'a String,
    items: Vec<String>,
    placeholder: &'a str,
    oninput: EventHandler<'a, String>,
) -> Element<'a> {
    let dropdown_class = classes!["dropdown-content bg-base-300 max-h-96 overflow-auto flex-col rounded-md z-10 w-full", "hidden"  => items.is_empty()];

    render! {
        div {
            class: "dropdown dropdown-start w-full {class}",
            input {
                class: "input input-primary w-full",
                r#type: "text",
                value: value.as_str(),
                autofocus: true,
                oninput: move |e| {
                    oninput.call(e.data.value.clone());
                },
                placeholder: *placeholder,
                tabindex: "0",
            }
            div {
                class: "{dropdown_class}",
                ul {
                    class: "menu",
                    for (i, item) in items.iter().enumerate() {
                        li {
                            key: "{i}",
                            tabindex: "{i + 1}",
                            button {
                                onclick: move |_| {
                                    oninput.call(item.clone());
                                },
                                "{item}"
                            }
                        }
                    }
                }
            }
        }
    }
}
