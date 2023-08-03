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
                class: "dropdown-content z-[1] card card-compact card-bordered border-base-300 shadow-lg shadow-black/50 bg-neutral text-neutral-content mt-1 min-w-[20em]",
                tabindex: 0,
                div {
                    class: "card-body",
                    tip
                }
            }
        }
    }
}
