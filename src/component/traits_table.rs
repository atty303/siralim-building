#![allow(non_snake_case)]

use classes::classes;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use fermi::use_read;
use std::ops::Range;

use data::stats::{ATTACK_RANGE, DEFENSE_RANGE, HEALTH_RANGE, INTELLIGENCE_RANGE, SPEED_RANGE};
use data::Data;

use crate::atom;
use crate::component::autocomplete::Autocomplete;
use crate::component::class_icon::ClassIcon;
use crate::component::description::Description;
use crate::component::outline_icon::OutlineIcon;

#[inline_props]
pub fn TraitsModal(cx: Scope) -> Element {
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
                        class: "whitespace-nowrap",
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
                        TraitsTable {
                            keys: keys.read().clone(),
                            selection: vec![],
                        }
                    }
                }
            }
    }
}

#[inline_props]
pub fn TraitsTable(cx: Scope, keys: Vec<i32>, selection: Vec<i32>) -> Element {
    let data: &Data = use_read(cx, &atom::DATA);
    let items = keys.iter().flat_map(|k| data.traits.get(k));

    render! {
        table {
            class: "table table-zebra table-pin-rows w-full",
            thead {
                tr {
                    th {}
                    th { "Class" }
                    th { "Family" }
                    th { "Creature" }
                    th { "Trait Description" }
                    th { class: "text-center", img { class: "inline-block", title: "Health", alt: "Health", src: "images/health.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Attack", alt: "Attack", src: "images/attack.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Intelligence", alt: "Intelligence", src: "images/intelligence.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Defense", alt: "Defense", src: "images/defense.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Speed", alt: "Speed", src: "images/speed.png" } }
                }
            }
            tbody {
                items.map(|t| {
                    let tr_class = classes!["group", "!bg-secondary/25" => selection.contains(&t.id)];
                    rsx! {
                        tr {
                            key: "{t.id}",
                            class: "{tr_class}",
                            td {
                                input {
                                    class: "checkbox checkbox-primary invisible group-hover:visible",
                                    r#type: "checkbox",
                                    checked: true,
                                }
                            }
                            td {
                                class: "whitespace-nowrap",
                                ClassIcon {
                                    class: "mr-1",
                                    name: t.class.as_str()
                                }
                                t.class.clone()
                            }
                            td {
                                class: "whitespace-nowrap",
                                t.family.clone()
                            }
                            td {
                                span {
                                    class: "underline decoration-dotted",
                                    t.creature.clone()
                                }
                            }
                            td {
                                class: "whitespace-normal",
                                Description {
                                    value: t.trait_description.clone(),
                                }
                            }
                            td { class: "text-center", StatsNumber { range: HEALTH_RANGE, value: t.health() } }
                            td { class: "text-center", StatsNumber { range: ATTACK_RANGE, value: t.attack() } }
                            td { class: "text-center", StatsNumber { range: INTELLIGENCE_RANGE, value: t.intelligence() } }
                            td { class: "text-center", StatsNumber { range: DEFENSE_RANGE, value: t.defense() } }
                            td { class: "text-center", StatsNumber { range: SPEED_RANGE, value: t.speed() } }
                        }
                    }
                })
            }
        }
    }
}

#[inline_props]
fn StatsNumber(cx: Scope, range: Range<u8>, #[props(!optional)] value: Option<u8>) -> Element {
    if let Some(value) = value {
        let base = value - range.start;
        let percent = base as f32 / range.len() as f32;

        let color = if percent < 0.5 {
            format!("rgba(239, 68, 68, {})", 1.0 - (percent * 2.0))
        } else {
            format!("rgba(34, 197, 94, {})", 1.0 - ((1.0 - percent) * 2.0))
        };

        render! {
            span {
                class: "p-2 rounded-md font-bold inline-block w-8",
                style: "background:{color}",
                "{value}"
            }
        }
    } else {
        render! {
            "-"
        }
    }
}
