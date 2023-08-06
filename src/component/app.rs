#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::component::footer::Footer;
use crate::component::navbar::NavBar;
use crate::component::party_member::PartyMember;
use crate::component::traits_table::TraitsModal;
use crate::embed_data::TRAITS_MAP;
use crate::hooks::drag::{use_dnd_context, DragEndEvent};
use crate::hooks::modal::use_modal;
use crate::hooks::persistent::use_persistent;
use crate::url_save;

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
    let trait_modal = use_modal(cx);

    let url_state = use_ref(cx, || {
        url_save::get_from_url().unwrap_or(Default::default())
    });

    use_effect(cx, &*url_state.read(), move |state| {
        to_owned![state];
        async move {
            url_save::set_to_url(&state);
        }
    });

    let show_traits = use_persistent(cx, "show_traits", || true);
    let show_spells = use_persistent(cx, "show_spells", || true);

    let traits_dnd_context = use_dnd_context::<TraitDndContext>(cx, {
        to_owned![url_state];
        Box::new(move |e: DragEndEvent| {
            log::debug!("DragEndEvent: {:?}", e);
            if let (Some(a), Some(o)) = (
                TraitDndContext::parse_id(e.active_id.as_str()),
                TraitDndContext::parse_id(e.over_id.as_str()),
            ) {
                log::debug!("a: {:?}, o: {:?}", a, o);

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

        traits_dnd_context.component::<TraitDndContext>(cx, render! {
            div {
                class: "mx-4 space-y-4",
                for (i, m) in url_state.read().party.iter().enumerate() {
                    PartyMember {
                        index: i,
                        member: m.clone(),
                        on_trait_click: move |trait_index| {
                            let us = url_state.clone();
                            trait_modal.show_modal(move |e| {
                                us.with_mut(|us| {
                                    us.party[i].traits[trait_index] = Some(&TRAITS_MAP[&e]);
                                });
                            });
                        },
                        on_trait_clear: move |trait_index| {
                            let us = url_state.clone();
                            us.with_mut(|us| {
                                us.party[i].traits[trait_index] = None;
                            });
                        },
                        show_traits: show_traits.clone(),
                        show_spells: show_spells.clone(),
                    }
                }
            }
            }
        )

        Footer {}

        trait_modal.component(cx, TraitsModal)
    }

    // cx.render(rsx! {
    //     div {
    //         DndState {
    //             Draggable {
    //                 DragHandle {
    //                     div { "Drag me" }
    //                 }
    //                 div { "outer" }
    //             }
    //             Droppable {
    //                 div { "Drop here" }
    //             }
    //         }
    //     }
    // })
}
