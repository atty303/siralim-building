#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::component::footer::Footer;
use crate::component::navbar::NavBar;
use crate::component::party_member::PartyMember;
use crate::component::personality_dialog::PersonalityDialog;
use crate::component::traits_table::TraitsModal;
use crate::embed_data::{PERSONALITIES_MAP, TRAITS_MAP};
use crate::hooks::drag::{use_dnd_context, DragEndEvent};
use crate::hooks::modal::use_modal;
use crate::hooks::persistent::use_persistent;
use crate::url_save;

#[derive(Debug)]
pub struct MemberDndContext {
    pub index: usize,
}

impl MemberDndContext {
    pub fn to_id(&self) -> String {
        format!("member:{}", self.index)
    }

    pub fn parse_id(id: &str) -> Option<Self> {
        let mut parts = id.split(':');
        if parts.next()? != "member" {
            return None;
        }
        let index = parts.next()?.parse().ok()?;
        Some(Self { index })
    }
}

#[derive(Debug)]
pub struct TraitDndContext {
    pub member_index: usize,
    pub trait_index: usize,
}

impl TraitDndContext {
    pub fn to_id(&self) -> String {
        format!("trait:{}:{}", self.member_index, self.trait_index)
    }

    pub fn parse_id(id: &str) -> Option<Self> {
        let mut parts = id.split(':');
        if parts.next()? != "trait" {
            return None;
        }
        let member_index = parts.next()?.parse().ok()?;
        let trait_index = parts.next()?.parse().ok()?;
        Some(Self {
            member_index,
            trait_index,
        })
    }
}

pub fn App(cx: Scope) -> Element {
    let url_state = use_ref(cx, || {
        url_save::get_from_url().unwrap_or(Default::default())
    });

    use_effect(cx, &*url_state.read(), move |state| {
        to_owned![state];
        async move {
            url_save::set_to_url(&state);
        }
    });

    let trait_modal = use_modal(cx, "w-[calc(100vw-5em)] max-w-full h-full".to_string());
    let personality_modal = use_modal(cx, "".to_string());

    let show_traits = use_persistent(cx, "show_traits", || true);
    let show_spells = use_persistent(cx, "show_spells", || true);

    let member_dnd_context = use_dnd_context::<MemberDndContext>(cx, {
        to_owned![url_state];
        Box::new(move |e: DragEndEvent| {
            if let (Some(a), Some(o)) = (
                MemberDndContext::parse_id(e.active_id.as_str()),
                MemberDndContext::parse_id(e.over_id.as_str()),
            ) {
                let us = url_state.clone();
                us.with_mut(|us| {
                    let tmp = us.party[a.index].clone();
                    us.party[a.index] = us.party[o.index].clone();
                    us.party[o.index] = tmp;
                });
            }
        })
    });

    let traits_dnd_context = use_dnd_context::<TraitDndContext>(cx, {
        to_owned![url_state];
        Box::new(move |e: DragEndEvent| {
            if let (Some(a), Some(o)) = (
                TraitDndContext::parse_id(e.active_id.as_str()),
                TraitDndContext::parse_id(e.over_id.as_str()),
            ) {
                let us = url_state.clone();
                us.with_mut(|us| {
                    let tmp = us.party[a.member_index].traits[a.trait_index];
                    us.party[a.member_index].traits[a.trait_index] =
                        us.party[o.member_index].traits[o.trait_index];
                    us.party[o.member_index].traits[o.trait_index] = tmp;
                });
            }
        })
    });

    render! {
        NavBar {
            url_state: url_state.clone()
        }

        h2 {
            class: "text-xl text-center text-secondary my-4",
            "PARTY"
        }

        member_dnd_context.component::<MemberDndContext>(cx, render! {
            traits_dnd_context.component::<TraitDndContext>(cx, render! {
                div {
                    class: "mx-4 space-y-4",
                    for (i, m) in url_state.read().party.iter().enumerate() {
                        PartyMember {
                            index: i,
                            member: m.clone(),
                            on_personality_click: move |_| {
                                to_owned![url_state];
                                personality_modal.show_modal(move |e| {
                                    url_state.with_mut(|us| {
                                        us.party[i].personality = Some(&PERSONALITIES_MAP[&e]);
                                    });
                                });
                            },
                            on_trait_click: move |trait_index| {
                                to_owned![url_state];
                                trait_modal.show_modal(move |e| {
                                    url_state.with_mut(|us| {
                                        us.party[i].traits[trait_index] = Some(&TRAITS_MAP[&e]);
                                    });
                                });
                            },
                            on_trait_clear: move |trait_index| {
                                to_owned![url_state];
                                url_state.with_mut(|us| {
                                    us.party[i].traits[trait_index] = None;
                                });
                            },
                            show_traits: show_traits.clone(),
                            show_spells: show_spells.clone(),
                        }
                    }
                }
            })
        })

        Footer {}

        trait_modal.component(cx, TraitsModal)
        personality_modal.component(cx, PersonalityDialog)
    }
}
