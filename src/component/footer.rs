#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    render! {
        footer {
            class: "footer p-4 mt-8 bg-neutral text-neutral-content",
            div {
                p {
                    "2023 Created by "
                    a {
                        class: "link",
                        href: "https://twitter.com/atty303",
                        "atty303"
                    }

                    ". This site is not affiliated with Thylacine Studios."
                }
                p {
                    "Data sourced from the "
                    a {
                        class: "link",
                        href: "https://docs.google.com/spreadsheets/d/1qvWwf1fNB5jN8bJ8dFGAVzC7scgDCoBO-hglwjTT4iY/edit#gid=0",
                        "Siralim Ultimate Compendium"
                    }
                    " and "
                    a {
                        class: "link",
                        href: "https://github.com/rovermicrover/siralim-ultimate-api",
                        "Siralim Ultimate API"
                    }
                    "."
                }
            }
        }
    }
}
