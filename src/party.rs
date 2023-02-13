#![allow(non_snake_case)]

use crate::components::ClassIcon;
use crate::data::Creature;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons;
use dioxus_free_icons::Icon;

#[derive(Props, PartialEq)]
pub struct PartyMemberProps {
    creature: (Option<Creature>, Option<Creature>, Option<Creature>),
}

pub fn PartyMember(cx: Scope<PartyMemberProps>) -> Element {
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
    let opacity = use_state(cx, || 1.0);
    let e = if let Some(c) = &cx.props.creature.0 {
        rsx! {
            div {
                class: "trait non-empty",
                "draggable": "true",
                style: "opacity: {opacity}",
                ondragstart: move |_| opacity.set(0.5),
                ondragend: move |_| opacity.set(1.0),
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
                class: "trait empty",
                cx.props.empty_text
            }
            div {
                class: "clear"
            }
        }
    };
    cx.render(e)
}
