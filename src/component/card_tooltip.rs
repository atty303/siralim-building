#![allow(non_snake_case)]

use dioxus::prelude::*;

#[inline_props]
pub fn CardTooltip<'a>(
    cx: Scope<'a>,
    tip: Element<'a>,
    class: Option<&'a str>,
    children: Element<'a>,
) -> Element<'a> {
    let hidden = use_state(cx, || true);
    render! {
        div {
            class: "dropdown dropdown-hover",
            onmouseenter: move |_| hidden.set(false),
            onmouseleave: move |_| hidden.set(true),
            div {
                class: "{class.unwrap_or_default()}",
                tabindex: 0,
                children
            }
            if !*hidden.get() {
                rsx! {
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
    }
}
