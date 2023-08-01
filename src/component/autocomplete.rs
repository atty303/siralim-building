#[allow(non_snake_case)]
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
    // let open = use_state(cx, || false);
    // let a = use_effect(cx, (items,), |(items,)| {
    //     to_owned![open];
    //     async move {
    //         open.set(!items.is_empty());
    //     }
    // });

    let dropdown_hidden = if items.is_empty() { "hidden" } else { "" };

    render! {
            div {
                class: "dropdown dropdown-start w-full {class}",
                input {
                    class: "input input-primary w-full",
                    r#type: "text",
                    value: value.as_str(),
                    oninput: move |e| {
                        oninput.call(e.data.value.clone());
    //                    open.set(true);
                    },
                    placeholder: *placeholder,
                    tabindex: "0",
                }
                div {
                    class: "dropdown-content bg-base-300 max-h-96 overflow-auto flex-col rounded-md z-10 w-full {dropdown_hidden}",
                    ul {
                        class: "menu",
                        for (i, item) in items.iter().enumerate() {
                            li {
                                key: "{i}",
                                tabindex: "{i + 1}",
                                button {
                                    oninput: move |e| {
                                        oninput.call(item.clone());
                                        // open.set(false);
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
