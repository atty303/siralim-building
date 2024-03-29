#![allow(non_snake_case)]

use std::ops::Range;

use classes::classes;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use data::r#trait::TraitId;
use data::stat::Stat;
use data::stats::{ATTACK_RANGE, DEFENSE_RANGE, HEALTH_RANGE, INTELLIGENCE_RANGE, SPEED_RANGE};

use crate::component::autocomplete::Autocomplete;
use crate::component::card_tooltip::CardTooltip;
use crate::component::class_icon::ClassIcon;
use crate::component::creature_card::CreatureCard;
use crate::component::description::Description;
use crate::component::outline_icon::OutlineIcon;
use crate::embed_data;
use crate::hooks::modal::ModalDialogProps;

pub fn TraitsModal<'a>(cx: Scope<'a, ModalDialogProps<'a, TraitId>>) -> Element<'a> {
    let keys = use_ref(cx, || vec![]);
    let total = embed_data::TRAITS_MAP.len();
    let autocomplete_items = use_ref(cx, || Vec::<String>::new());

    let query = use_state(cx, || String::from(""));

    use_effect(cx, (query,), |(query,)| {
        to_owned![keys, autocomplete_items];
        async move {
            let (results, complements) = if query.len() > 1 {
                let r = embed_data::TRAITS_INDEX.search(query.as_str());
                let c = embed_data::TRAITS_INDEX
                    .autocomplete(query.as_str())
                    .into_iter()
                    .filter(|i| query.trim() != *i)
                    .collect();
                (r, c)
            } else {
                (vec![], vec![])
            };
            keys.set(results.iter().map(|i| **i).collect());
            autocomplete_items.set(complements);
        }
    });

    render! {
        //div { class: "max-h-64 h-64 overflow-hidden",
        div {
            class: "flex flex-col gap-4 min-h-full max-h-full",
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
                    if !keys.read().is_empty() {
                        rsx! {
                            TraitsTable {
                                keys: keys.read().clone(),
                                selection: vec![],
                                on_select: |id| cx.props.on_result.call(id),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
pub fn TraitsTable<'a>(
    cx: Scope<'a>,
    keys: Vec<TraitId>,
    selection: Vec<TraitId>,
    on_select: EventHandler<'a, TraitId>,
) -> Element<'a> {
    let items = keys.iter().flat_map(|k| embed_data::TRAITS_MAP.get(k));

    render! {
        table {
            class: "table table-zebra table-pin-rows w-full",
            thead {
                tr {
                    class: "bg-neutral text-neutral-content",
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
                                    prevent_default: "onclick",
                                    onclick: move |_| on_select.call(t.id),
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
                                CardTooltip {
                                    tip: render! { CreatureCard { r#trait: t } },
                                    class: "underline decoration-dotted",
                                    "{t.creature}"
                                }

                            }
                            td {
                                class: "whitespace-normal",
                                Description {
                                    value: t.trait_description.clone(),
                                }
                            }
                            td { class: "text-center", StatsNumber { range: HEALTH_RANGE, value: t.stat(Stat::Health) } }
                            td { class: "text-center", StatsNumber { range: ATTACK_RANGE, value: t.stat(Stat::Attack) } }
                            td { class: "text-center", StatsNumber { range: INTELLIGENCE_RANGE, value: t.stat(Stat::Intelligence) } }
                            td { class: "text-center", StatsNumber { range: DEFENSE_RANGE, value: t.stat(Stat::Defense) } }
                            td { class: "text-center", StatsNumber { range: SPEED_RANGE, value: t.stat(Stat::Speed) } }
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
