#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    render! {
        footer {
            class: "bg-neutral text-neutral-content mt-4 p-4",
            "2023 Created by atty303. This site is not affiliated with Thylacine Studios."
            br {}
            "Data sourced from the"
            a {
                href: "https://docs.google.com/spreadsheets/d/1qvWwf1fNB5jN8bJ8dFGAVzC7scgDCoBO-hglwjTT4iY/edit#gid=0",
                "Siralim Ultimate Compendium"
            }
            " and "
            a {
                href: "https://github.com/rovermicrover/siralim-ultimate-api",
                "Siralim Ultimate API"
            }
            "."
        }
    }
}
