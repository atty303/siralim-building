#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn CardTooltip<'a>(
    cx: Scope<'a>,
    tip: Element<'a>,
    class: Option<&'a str>,
    children: Element<'a>,
) -> Element<'a> {
    render! {
        div {
            class: "dropdown dropdown-hover",
            label {
                class: "{class.unwrap_or_default()}",
                tabindex: 0,
                children
            }
            div {
                class: "dropdown-content z-[1] min-w-max card card-compact shadow bg-neutral text-neutral-content mt-1",
                tabindex: 0,
                div {
                    class: "card-body",
                    tip
                }
            }
        }
    }
}
