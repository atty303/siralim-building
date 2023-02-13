#![allow(non_snake_case)]

use crate::components::ClassIcon;
use crate::data::Creature;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons;
use dioxus_free_icons::Icon;

#[derive(Clone, PartialEq)]
pub struct Member {
    pub primary_creature: Option<Creature>,
    pub fused_creature: Option<Creature>,
    pub artifact_creature: Option<Creature>,
}
impl Member {
    pub fn get_creature(&self, i: i32) -> &Option<Creature> {
        match i {
            0 => &self.primary_creature,
            1 => &self.fused_creature,
            2 => &self.artifact_creature,
            _ => &None,
        }
    }
    pub fn set_creature(&mut self, i: i32, c: &Option<Creature>) {
        match i {
            0 => self.primary_creature = c.clone(),
            1 => self.fused_creature = c.clone(),
            2 => self.artifact_creature = c.clone(),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub struct PartyMemberSwapEvent {
    pub from_position: i32,
    pub from_index: i32,
    pub to_position: i32,
    pub to_index: i32,
}

#[derive(Props)]
pub struct PartyProps<'a> {
    party: &'a Vec<Member>,
    on_swap: EventHandler<'a, PartyMemberSwapEvent>,
}

pub fn Party<'a>(cx: Scope<'a, PartyProps<'a>>) -> Element {
    let dragging: &UseState<Option<PartyMemberTraitDragEvent>> = use_state(cx, || None);

    cx.render(rsx! {
        div {
            class: "party",
            cx.props.party.iter().enumerate().map(|(i, m)| {
                rsx! {
                    PartyMember {
                        position: i as i32
                        creature: (m.primary_creature.clone(), m.fused_creature.clone(), m.artifact_creature.clone())
                        on_drag_start: move |e| {
                            dragging.set(Some(e));
                        }
                        on_drop: move |e: PartyMemberTraitDropEvent| {
                            if let Some(a) = dragging.get() {
                                cx.props.on_swap.call(PartyMemberSwapEvent {
                                    from_position: a.position,
                                    from_index: a.index,
                                    to_position: e.position,
                                    to_index: e.index,
                                })
                            }
                            dragging.set(None);
                        }
                    }
                }
            })
        }
    })
}

struct PartyMemberTraitDragEvent {
    position: i32,
    index: i32,
    creature: Creature,
}

struct PartyMemberTraitDropEvent {
    position: i32,
    index: i32,
}

#[derive(Props)]
pub struct PartyMemberProps<'a> {
    position: i32,
    creature: (Option<Creature>, Option<Creature>, Option<Creature>),
    on_drag_start: EventHandler<'a, PartyMemberTraitDragEvent>,
    on_drop: EventHandler<'a, PartyMemberTraitDropEvent>,
}

pub fn PartyMember<'a>(cx: Scope<'a, PartyMemberProps<'a>>) -> Element {
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
                            empty_text: "Click to add primary trait"
                            on_drag_start: move |e| cx.props.on_drag_start.call(PartyMemberTraitDragEvent {
                                position: cx.props.position,
                                index: 0,
                                creature: e,
                            })
                            on_drop: move |_| cx.props.on_drop.call(PartyMemberTraitDropEvent {
                                position: cx.props.position,
                                index: 0,
                            })
                        }
                    }
                    li {
                        PartyMemberTrait {
                            creature: (&cx.props.creature.1,)
                            empty_text: "Click to add fused trait"
                            on_drag_start: move |e| cx.props.on_drag_start.call(PartyMemberTraitDragEvent {
                                position: cx.props.position,
                                index: 1,
                                creature: e,
                            })
                            on_drop: move |_| cx.props.on_drop.call(PartyMemberTraitDropEvent {
                                position: cx.props.position,
                                index: 0,
                            })
                        }
                    }
                    li {
                        PartyMemberTrait {
                            creature: (&cx.props.creature.2,)
                            empty_text: "Click to add artifact trait"
                            on_drag_start: move |e| cx.props.on_drag_start.call(PartyMemberTraitDragEvent {
                                position: cx.props.position,
                                index: 2,
                                creature: e,
                            })
                            on_drop: move |_| cx.props.on_drop.call(PartyMemberTraitDropEvent {
                                position: cx.props.position,
                                index: 0,
                            })
                        }
                    }
                }
            }
        }
    })
}

#[derive(Props)]
struct PartyMemberTraitProps<'a> {
    creature: (&'a Option<Creature>,),
    empty_text: &'static str,
    on_drag_start: EventHandler<'a, Creature>,
    on_drop: EventHandler<'a>,
}

fn PartyMemberTrait<'a>(cx: Scope<'a, PartyMemberTraitProps<'a>>) -> Element {
    let opacity = use_state(cx, || 1.0);
    let e = if let Some(c) = &cx.props.creature.0 {
        rsx! {
            div {
                class: "trait non-empty",
                "draggable": "true",
                style: "opacity: {opacity}",
                onclick: move |e| {
                    log::debug!("onclick: {:?}", e);
                },
                ondragstart: move |e| {
                    log::debug!("ondragstart: {:?}", e);
                    opacity.set(0.5);
                    cx.props.on_drag_start.call(c.clone());
                },
                ondragend: move |_| opacity.set(1.0),
                ondrop: move |e| {
                    log::debug!("ondrop: {:?}", e);
                    cx.props.on_drop.call(());
                },
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
