#![allow(non_snake_case)]

mod components;
mod creatures;
mod data;

use crate::components::ClassIcon;
use crate::creatures::CreatureModal;
use crate::data::Creature;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons;
use dioxus_free_icons::Icon;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let creatures = data::Data::creatures();

    let show_creatures_modal = use_state(cx, || true);

    cx.render(rsx! {
        div {
            "header"
            button {
                onclick: move |_| show_creatures_modal.set(true),
                "open"
            }
            div {
                class: "party",
                PartyMember {
                    creature: (Some(creatures.get(0).unwrap().clone()), None, None)
                }
            }
        }
        CreatureModal {
            items: creatures,
            show: **show_creatures_modal,
            on_select: move |_| {
                show_creatures_modal.set(false);
            }
        }
        // Modal {
        //     show: **show_creatures_modal,
        //     on_request_close: move |_| show_creatures_modal.set(false),
        //     input { }
        //     CreatureTable {
        //         items: creatures
        //         on_select: move |e|
        //     }
        // }
    })
}

#[derive(Props, PartialEq)]
struct PartyMemberProps {
    creature: (Option<Creature>, Option<Creature>, Option<Creature>),
}

fn PartyMember(cx: Scope<PartyMemberProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "party-member",
            div {
                class: "left-pane",
                "x"
            }
            div {
                class: "right-pane",
                ul {
                    li {
                        PartyMemberTrait {
                            creature: (&cx.props.creature.0,)
                            empty_text: "Click to add primary trait",
                        }
                    }
                    li {
                        PartyMemberTrait {
                            creature: (&cx.props.creature.1,)
                            empty_text: "Click to add fused trait",
                        }
                    }
                    li {
                        PartyMemberTrait {
                            creature: (&cx.props.creature.2,)
                            empty_text: "Click to add artifact trait",
                        }
                    }
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct PartyMemberTraitProps<'a> {
    creature: (&'a Option<Creature>,),
    empty_text: &'static str,
}

fn PartyMemberTrait<'a>(cx: Scope<'a, PartyMemberTraitProps<'a>>) -> Element {
    let e = if let Some(c) = &cx.props.creature.0 {
        rsx! {
            div {
                class: "detail non-empty",
                div {
                    class: "creature",
                    span { ClassIcon { value: &c.class } c.creature.as_str() }
                }
                div {
                    class: "trait-name",
                    span { c.trait_name.as_str() }
                }
                div {
                    class: "trait-description",
                    span { c.trait_description.as_str() }
                }
            }
            div {
                class: "clear",
                button {
                    Icon { width: 24, height: 24, icon: bs_icons::BsXLg }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "detail empty",
                cx.props.empty_text
            }
            div {
                class: "clear"
            }
        }
    };
    cx.render(e)
}
