use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use data::personality::PersonalityId;
use data::stat::Stat;

use crate::component::outline_icon::OutlineIcon;
use crate::embed_data;
use crate::hooks::modal::ModalDialogProps;

#[allow(non_snake_case)]
pub fn PersonalityDialog<'a>(cx: Scope<'a, ModalDialogProps<'a, PersonalityId>>) -> Element<'a> {
    render! {
        div {
            PersonalityDialogTable {
                on_result: |e| cx.props.on_result.call(e),
            }
        }
    }
}

#[allow(non_snake_case)]
fn PersonalityDialogTable<'a>(cx: Scope<'a, ModalDialogProps<'a, PersonalityId>>) -> Element<'a> {
    render! {
        table {
            class: "table table-zebra table-xs",
            thead {
                tr {
                    class: "bg-neutral text-neutral-content",
                    th {}
                    th { "Personality" }
                    th { class: "text-center", img { class: "inline-block", title: "Health", alt: "Health", src: "images/health.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Attack", alt: "Attack", src: "images/attack.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Intelligence", alt: "Intelligence", src: "images/intelligence.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Defense", alt: "Defense", src: "images/defense.png" } }
                    th { class: "text-center", img { class: "inline-block", title: "Speed", alt: "Speed", src: "images/speed.png" } }
                }
            }
            tbody {
                embed_data::PERSONALITIES_MAP.values().map(|r| {
                    rsx! {
                        tr {
                            key: "{r.id}",
                            class: "group",
                            td {
                                class: "text-center",
                                input {
                                    class: "checkbox checkbox-primary invisible group-hover:visible",
                                    r#type: "checkbox",
                                    checked: true,
                                    prevent_default: "onclick",
                                    onclick: move |_| cx.props.on_result.call(r.id),
                                }
                            }
                            td {
                                class: "whitespace-nowrap",
                                r.name
                            }
                            PersonalityStat {
                                stat: &Stat::Health,
                                positive: &r.positive,
                                negative: &r.negative,
                            }
                            PersonalityStat {
                                stat: &Stat::Attack,
                                positive: &r.positive,
                                negative: &r.negative,
                            }
                            PersonalityStat {
                                stat: &Stat::Intelligence,
                                positive: &r.positive,
                                negative: &r.negative,
                            }
                            PersonalityStat {
                                stat: &Stat::Defense,
                                positive: &r.positive,
                                negative: &r.negative,
                            }
                            PersonalityStat {
                                stat: &Stat::Speed,
                                positive: &r.positive,
                                negative: &r.negative,
                            }
                        }
                    }
                })
            }
        }
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn PersonalityStat<'a>(
    cx: Scope,
    stat: &'a Stat,
    positive: &'a Stat,
    negative: &'a Stat,
) -> Element {
    if stat == positive {
        render! {
            td {
                class: "p-2",
                div {
                    class: "text-center rounded-md bg-success text-success-content",
                    OutlineIcon {
                        icon: Shape::ChevronUp,
                        size: 16,
                    }
                }
            }
        }
    } else if stat == negative {
        render! {
            td {
                class: "p-2",
                div {
                    class: "text-center rounded-md bg-error text-error-content",
                    OutlineIcon {
                        icon: Shape::ChevronDown,
                        size: 16,
                    }
                }
            }
        }
    } else {
        render! {
            td {
            }
        }
    }
}
