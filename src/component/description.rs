#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::component::card_tooltip::CardTooltip;
use crate::embed_data::{EFFECTS_MAP, KEYWORDS_MAP};

#[inline_props]
pub fn Description(cx: Scope, value: Vec<String>) -> Element {
    render! {
        span {
            value.iter().map(|t| {
                if let Some(w) = KEYWORDS_MAP.get(t) {
                    rsx! {
                        span {
                            class: "font-bold text-accent",
                            img {
                                class: "inline-block w-4 h-4 mr-1",
                                src: "images/{w.icon}",
                            }
                            "{t}"
                        }
                    }
                } else if let Some(e) = EFFECTS_MAP.get(t) {
                    let color = match e.category.as_str() {
                        "buff" => "text-success",
                        "debuff" => "text-error",
                        _ => "text-warning",
                    };
                    rsx! {
                        CardTooltip {
                            tip: render! { p { "{e.description}" } },
                            class: "font-bold underline decoration-dotted {color}",
                            img {
                                class: "inline-block w-4 h-4 mr-1",
                                src: "status_icons/{e.icon}",
                            }
                            "{t}"
                        }
                    }
                } else {
                    // TODO: spell
                    rsx! {
                        span {
                            "{t}"
                        }
                    }
                }
            })
        }
    }
}
