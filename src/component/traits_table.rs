#[allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use fermi::use_read;
use std::ops::Deref;

use data::Data;

use crate::atom;
use crate::component::autocomplete::Autocomplete;
use crate::component::modal::Modal;
use crate::component::outline_icon::OutlineIcon;

#[inline_props]
pub fn TraitsModal(cx: Scope, show: bool) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);
    let index = data.traits_index.clone();
    let keys = use_ref(cx, || vec![]);
    let total = data.traits.len();
    let autocomplete_items = use_ref(cx, || Vec::<String>::new());

    let query = use_state(cx, || String::from(""));

    use_effect(cx, (query,), |(query,)| {
        to_owned![keys, autocomplete_items];
        async move {
            // log::debug!("search: {}", query);
            let results = index.search(query.as_str());
            let complements = index
                .autocomplete(query.as_str())
                .into_iter()
                .filter(|i| query.trim() != *i)
                .collect();
            // log::debug!("completes: {:?}", complements);
            keys.set(results.iter().map(|i| **i).collect());
            autocomplete_items.set(complements);
        }
    });

    render! {
        Modal {
            show: show,
            on_request_close: move |_| {},
            //div { class: "max-h-64 h-64 overflow-hidden",
            div {
                class: "flex flex-col gap-4 max-h-full",
                div {
                    class: "flex-initial w-full flex items-center space-x-4",
                    button {
                        class: "btn btn-primary",
                        OutlineIcon {
                            icon: Shape::Bookmark,
                        }
                    }
                    // input {
                    //     class: "input input-primary grow",
                    //     r#type: "text",
                    //     placeholder: "Search monster/traits...",
                    //     oninput: move |e| {
                    //         let value = e.value.clone();
                    //         query.set(e.value.clone());
                    //     }
                    // }
                    Autocomplete {
                        class: "grow",
                        value: query,
                        items: autocomplete_items.read().clone(),
                        placeholder: "Search monster/traits...",
                        oninput: move |value| {
                            query.set(value);
                        }
                    }
                    div {
                        span {
                            class: "font-bold",
                            format!("{}", keys.read().len())
                        }
                        " of "
                        span {
                            class: "font-bold",
                            format!("{}", total)
                        }
                        " results"
                    }
                }
                div {
                    class: "grow overflow-y-auto",
                    div {
                        class: "",
                        TraitsTable {
                            keys: keys.read().clone(),
                        }
                    }
                }
            }
            //    }
        }
    }
}

#[inline_props]
pub fn TraitsTable(cx: Scope, keys: Vec<i32>) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);
    let items = keys.iter().flat_map(|k| data.traits.get(k));

    render! {
        table {
            class: "table table-zebra table-pin-rows w-full",
            thead {
                tr {
                    // class: "sticky top-0",
                    th { "Class" }
                    th { "Family" }
                    th { "Creature" }
                    th { "Trait Description" }
                    th { img { src: "images/health.png" } }
                    th { img { src: "images/attack.png" } }
                    th { img { src: "images/intelligence.png" } }
                    th { img { src: "images/defense.png" } }
                    th { img { src: "images/speed.png" } }
                }
            }
            tbody {
                for t in items {
                    tr {
                        td { t.class.clone() }
                        td { t.family.clone() }
                        td { t.creature.clone() }
                        td {
                            class: "whitespace-normal",
                            t.trait_description.join(" ")
                        }
                        td { t.health().map(|v| v.to_string()).unwrap_or("-".to_string()) }
                        td { t.attack().map(|v| v.to_string()).unwrap_or("-".to_string()) }
                        td { t.intelligence().map(|v| v.to_string()).unwrap_or("-".to_string()) }
                        td { t.defense().map(|v| v.to_string()).unwrap_or("-".to_string()) }
                        td { t.speed().map(|v| v.to_string()).unwrap_or("-".to_string()) }
                    }
                }
            }
        }
    }
}
