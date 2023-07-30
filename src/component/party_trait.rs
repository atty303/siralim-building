#![allow(non_snake_case)]

use data::r#trait::Trait;
use dioxus::prelude::*;

#[inline_props]
pub fn PartyTrait(cx: Scope, r#trait: Option<Trait>, empty_text: &'static str) -> Element {
    render! {
        div {
            class: "flex flex-col items-stretch",
            if let Some(t) = r#trait {
                rsx! {
                    div {
                        class: "flex grow items-center",
                        ""
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "flex grow items-center text-center",
                        span {
                            class: "grow",
                            empty_text.to_string()
                        }
                    }
                }
            }
        }
    }
}
