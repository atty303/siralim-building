use dioxus::prelude::*;
use fermi::use_read;

use data::Data;

use crate::atom;
use crate::component::modal::Modal;

#[inline_props]
pub fn TraitsModal(cx: Scope, show: bool) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);
    let index = data.traits_index.clone();
    let keys = use_ref(cx, || vec![]);

    let query = use_state(cx, || String::from(""));

    use_effect(cx, (query,), |(query,)| {
        to_owned![keys];
        async move {
            log::debug!("search: {}", query);
            let results = index.search(query.as_str());
            let complements = index.autocomplete(query.as_str());
            log::debug!("completes: {:?}", complements);
            keys.set(results.iter().map(|i| **i).collect());
        }
    });

    render! {
        Modal {
            show: show,
            on_request_close: move |_| {},
            div {
                div {
                    class: "w-full flex items-center",
                    input {
                        class: "input grow",
                        r#type: "text",
                        placeholder: "Search monster/traits...",
                        oninput: move |e| {
                            let value = e.value.clone();
                            query.set(e.value.clone());
                            // let results = index.search(value.as_str());
                            // keys.set(results.iter().map(|i| **i).collect());
                        }
                    }
                    span {
                        format!("{} items", keys.read().len())
                    }
                }
            }
            div {
                TraitsTable {
                    keys: keys.read().clone(),
                }
            }
        }
    }
}

#[inline_props]
pub fn TraitsTable(cx: Scope, keys: Vec<i32>) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);
    let items = keys.iter().flat_map(|k| data.traits.get(k));

    render! {
        table {
            class: "table w-full max-w-full",
            thead {
                tr {
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
                        td { t.trait_description.join(" ") }
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
