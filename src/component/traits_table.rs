use dioxus::prelude::*;
use fermi::use_read;

use data::Data;

use crate::atom;

pub fn TraitsModal<'a>(cx: Scope<'a>) -> Element<'a> {
    let data: &Data = use_read(cx, &atom::DATA);
    let index = data.traits_index.clone();
    let keys = use_ref(cx, || vec![]);

    render! {
        div {
            div {
                input {
                    r#type: "text",
                    oninput: move |e| {
                        let value = e.value.clone();
                        let results = index.search(value.as_str());
                        keys.set(results.iter().map(|i| **i).collect());
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

#[inline_props]
pub fn TraitsTable(cx: Scope, keys: Vec<i32>) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);
    let items = keys.iter().flat_map(|k| data.traits.get(k));

    render! {
        table {
            class: "table",
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
