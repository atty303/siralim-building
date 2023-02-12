#![allow(non_snake_case)]

use crate::components::{ClassIcon, Modal};
use crate::data::Creature;
use dioxus::prelude::*;

pub struct CreatureSelectEvent {
    pub creature: Option<Creature>,
}

#[derive(Props)]
pub struct CreatureModalProps<'a> {
    items: Vec<Creature>,
    show: bool,
    on_select: EventHandler<'a, CreatureSelectEvent>,
}

pub fn CreatureModal<'a>(cx: Scope<'a, CreatureModalProps<'a>>) -> Element {
    cx.render(rsx! {
        Modal {
            show: cx.props.show,
            on_request_close: move |_| cx.props.on_select.call(CreatureSelectEvent { creature: None }),
            CreatureTable {
                items: cx.props.items.to_vec(),
                on_select: move |e| cx.props.on_select.call(e)
            }
        }
    })
}

#[derive(Props)]
pub struct CreatureTableProps<'a> {
    items: Vec<Creature>,
    on_select: EventHandler<'a, CreatureSelectEvent>,
}

pub fn CreatureTable<'a>(cx: Scope<'a, CreatureTableProps<'a>>) -> Element {
    cx.render(rsx! {
        table {
            class: "creatures",
            thead {
                tr {
                    th { "Class" }
                    th { "Family" }
                    th { "Creature" }
                    th { "Trait" }
                    th { "Trait Description" }
                    th { img { src: "image/health.png" } }
                    th { img { src: "image/attack.png" } }
                    th { img { src: "image/intelligence.png" } }
                    th { img { src: "image/defense.png" } }
                    th { img { src: "image/speed.png" } }
                    th { "Material Name" }
                }
            }
            tbody {
                cx.props.items.iter().map(|c|
                    rsx! {
                        tr {
                            onclick: move |_| {
                                cx.props.on_select.call(CreatureSelectEvent { creature: Some(c.clone()) });
                            },
                            td { class: "class", CreatureClass { value: c.class.clone() } }
                            td { class: "family", c.family.as_str() }
                            td { class: "creature", c.creature.as_str() }
                            td { class: "trait", c.trait_name.as_str() }
                            td { class: "trait_description", c.trait_description.as_str() }
                            td {
                                class: "stat health",
                                c.stats.as_ref().map(|s| rsx! { CreatureStat { value: s.health } })
                            }
                            td {
                                class: "stat attack",
                                c.stats.as_ref().map(|s| rsx! { CreatureStat { value: s.attack } })
                            }
                            td {
                                class: "stat intelligence",
                                c.stats.as_ref().map(|s| rsx! { CreatureStat { value: s.intelligence } })
                            }
                            td {
                                class: "stat defense",
                                c.stats.as_ref().map(|s| rsx! { CreatureStat { value: s.defense } })
                            }
                            td {
                                class: "stat speed",
                                c.stats.as_ref().map(|s| rsx! { CreatureStat { value: s.speed } })
                            }
                            td { class: "material_name", c.material_name.as_str() }
                        }
                    }
                )
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct CreatureClassProps {
    value: String,
}

fn CreatureClass(cx: Scope<CreatureClassProps>) -> Element {
    cx.render(rsx! { ClassIcon { value: &cx.props.value }, cx.props.value.as_str() })
}

#[derive(PartialEq, Props)]
pub struct CreatureStatProps {
    value: i32,
}

fn CreatureStat(cx: Scope<CreatureStatProps>) -> Element {
    cx.render(rsx! { "{cx.props.value}" })
}
