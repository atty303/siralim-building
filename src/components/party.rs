use std::ops::Deref;

use yew::prelude::*;
use yew_icons::{Icon, IconId};

use data::r#trait::Trait;

use crate::components::icon::ClassIcon;
use crate::state::Member;

#[derive(Debug)]
pub struct PartySwapEvent {
    pub from_position: usize,
    pub from_index: usize,
    pub to_position: usize,
    pub to_index: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PartyTraitEvent {
    pub position: usize,
    pub index: usize,
}
impl PartyTraitEvent {
    fn new(position: usize, index: usize) -> PartyTraitEvent {
        PartyTraitEvent { position, index }
    }
}

#[derive(Properties, PartialEq)]
pub struct PartyProps {
    pub party: Vec<Member>,
    pub on_swap: Callback<PartySwapEvent>,
    pub on_click: Callback<PartyTraitEvent>,
}

#[function_component(Party)]
pub fn party(props: &PartyProps) -> Html {
    let dragging: UseStateHandle<Option<PartyTraitEvent>> = use_state(|| None);
    html! {
        <div class="party">
            {props.party.iter().enumerate().map(|(i, m)| {
                let on_click = {
                    let on_click = props.on_click.clone();
                    Callback::from(move |e: PartyTraitEvent| {
                        on_click.emit(e.clone());
                    })
                };
                let on_drag_start = {
                    let dragging = dragging.clone();
                    Callback::from(move |e: PartyTraitEvent| {
                        log::debug!("dragstart: {:?}", e);
                        dragging.set(Some(e));
                    })
                };
                let on_drop = {
                    let dragging = dragging.clone();
                    let on_swap = props.on_swap.clone();
                    Callback::from(move |e: PartyTraitEvent| {
                        log::debug!("drop: {:?}", e);
                        if let Some(a) = dragging.as_ref() {
                            on_swap.emit(PartySwapEvent {
                                from_position: a.position,
                                from_index: a.index,
                                to_position: e.position,
                                to_index: e.index,
                            })
                        }
                        dragging.set(None);
                    })
                };
                html! {
                    <PartyMember
                        position={i}
                        member={m.clone()}
                        on_click={on_click}
                        on_drag_start={on_drag_start}
                        on_drop={on_drop}
                    >
                    </PartyMember>
                }
            }).collect::<Html>()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PartyMemberProps {
    position: usize,
    member: Member,
    on_click: Callback<PartyTraitEvent>,
    on_drag_start: Callback<PartyTraitEvent>,
    on_drop: Callback<PartyTraitEvent>,
}

#[function_component(PartyMember)]
fn party_member(props: &PartyMemberProps) -> Html {
    let on_click = |index: usize| {
        let on_click = props.on_click.clone();
        let e = PartyTraitEvent::new(props.position, index);
        Callback::from(move |_| {
            on_click.emit(e.clone());
        })
    };
    let on_drag_start = |index: usize| {
        let on_drag_start = props.on_drag_start.clone();
        let e = PartyTraitEvent::new(props.position, index);
        Callback::from(move |_| {
            log::debug!("ondragstart");
            on_drag_start.emit(e.clone());
        })
    };
    let on_drop = |index: usize| {
        let on_drop = props.on_drop.clone();
        let e = PartyTraitEvent::new(props.position, index);
        Callback::from(move |_| {
            log::debug!("ondrop");
            on_drop.emit(e.clone());
        })
    };

    html! {
        <div class="party-member">
            <div class="left-pane"></div>
            <div class="right-pane">
                <ul>
                    <li>
                        <PartyTrait
                            r#trait={props.member.primary_trait.clone()}
                            empty_text={"Click to add primary trait"}
                            on_click={on_click(0).clone()}
                            on_drag_start={on_drag_start(0).clone()}
                            on_drop={on_drop(0).clone()}
                        />
                    </li>
                    <li>
                        <PartyTrait
                            r#trait={props.member.fused_trait.clone()}
                            empty_text={"Click to add fused trait"}
                            on_click={on_click(1).clone()}
                            on_drag_start={on_drag_start(1).clone()}
                            on_drop={on_drop(1).clone()}
                        />
                    </li>
                    <li>
                        <PartyTrait
                            r#trait={props.member.artifact_trait.clone()}
                            empty_text={"Click to add artifact trait"}
                            on_click={on_click(2).clone()}
                            on_drag_start={on_drag_start(2).clone()}
                            on_drop={on_drop(2).clone()}
                        />
                    </li>
                </ul>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PartyTraitProps {
    r#trait: Option<Trait>,
    empty_text: &'static str,
    on_click: Callback<()>,
    on_drag_start: Callback<()>,
    on_drop: Callback<()>,
}

#[function_component(PartyTrait)]
fn party_trait(props: &PartyTraitProps) -> Html {
    let opacity = use_state(|| 1.0);

    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    let ondragstart = {
        let on_drag_start = props.on_drag_start.clone();
        let opacity = opacity.clone();
        Callback::from(move |_| {
            opacity.set(0.5);
            on_drag_start.emit(());
        })
    };

    let ondragend = {
        let opacity = opacity.clone();
        Callback::from(move |_| opacity.set(1.0))
    };

    let ondragover = { Callback::from(move |e: DragEvent| e.prevent_default()) };

    let ondrop = {
        let on_drop = props.on_drop.clone();
        Callback::from(move |e| {
            log::debug!("ondrop: {:?}", e);
            on_drop.emit(());
        })
    };

    if let Some(c) = &props.r#trait {
        html! {
            <>
                <div
                    class="trait non-empty"
                    style={format!("opacity: {}", opacity.deref())}
                    draggable="true"
                    onclick={onclick}
                    ondragstart={ondragstart}
                    ondragend={ondragend}
                    ondragover={ondragover}
                    ondrop={ondrop}
                >
                    <div class="creature">
                        <span>
                            <ClassIcon value={c.class.clone()} />
                            {c.creature.as_str()}
                        </span>
                    </div>
                    <div class="trait-name">
                        <span>{c.trait_name.as_str()}</span>
                    </div>
                    <div class="trait-description">
                        <span>{c.trait_description.as_str()}</span>
                    </div>
                </div>
                <div class="clear">
                    <button><Icon icon_id={IconId::BootstrapXLg} /></button>
                </div>
            </>
        }
    } else {
        html! {
            <>
                <div
                    class="trait empty"
                    onclick={onclick}
                    ondragover={ondragover}
                    ondrop={ondrop}
                >
                    {props.empty_text}
                </div>
                <div class="clear"></div>
            </>
        }
    }
}
