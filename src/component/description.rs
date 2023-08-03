#![allow(non_snake_case)]

use crate::atom;
use data::Data;
use dioxus::prelude::*;
use fermi::use_read;

#[inline_props]
pub fn Description(cx: Scope, value: Vec<String>) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);

    render! {
        span {
            value.iter().map(|t| {
                if let Some(w) = data.keywords.get(t) {
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
                } else if let Some(e) = data.effects.get(t) {
                    let color = match e.category.as_str() {
                        "buff" => "text-success",
                        "debuff" => "text-error",
                        _ => "text-warning",
                    };
                    rsx! {
                        div {
                            class: "dropdown dropdown-hover",
                            label {
                                class: "font-bold underline decoration-dotted {color}",
                                tabindex: 0,
                                img {
                                    class: "inline-block w-4 h-4 mr-1",
                                    src: "status_icons/{e.icon}",
                                }
                                "{t}"
                            }
                            div {
                                class: "dropdown-content z-[1] card card-compact w-96 shadow bg-neutral text-neutral-content mt-1",
                                tabindex: 0,
                                div {
                                    class: "card-body",
                                    p {
                                        "{e.description}"
                                    }
                                }
                            }
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
