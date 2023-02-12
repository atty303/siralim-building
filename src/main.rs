#![allow(non_snake_case)]

mod creatures;
mod data;

use crate::creatures::{CreatureSelectEvent, CreatureTable};
use crate::data::Creature;
use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("failed to init logger");
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let creatures = data::Data::creatures();

    let mut show_creatures_modal = use_state(cx, || false);

    cx.render(rsx! {
        div {
            "header"
            button {
                onclick: move |_| show_creatures_modal.set(true),
                "open"
            }
            PartyMember {
                creature: (Some(creatures.get(0).unwrap().clone()), None, None)
            }
        }
        CreatureModal {
            items: creatures,
            show: **show_creatures_modal,
            on_select: move |e| {
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

#[derive(Props)]
struct ModalProps<'a> {
    show: bool,
    on_request_close: EventHandler<'a, ()>,
    children: Element<'a>,
}

fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element {
    if cx.props.show {
        cx.render(rsx! {
            button { onclick: move |_|  cx.props.on_request_close.call(()), "close" }
            div {
                &cx.props.children
            }
        })
    } else {
        None
    }
}

#[derive(Props)]
struct CreatureModalProps<'a> {
    items: Vec<Creature>,
    show: bool,
    on_select: EventHandler<'a, CreatureSelectEvent>,
}

fn CreatureModal<'a>(cx: Scope<'a, CreatureModalProps<'a>>) -> Element {
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
                class: "detail",
                c.creature.as_str(),
            }
            div {
                class: "clear",
                button {
                    "X"
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "detail",
                cx.props.empty_text
            }
            div {
                class: "clear"
            }
        }
    };
    cx.render(e)
}
