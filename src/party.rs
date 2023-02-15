use crate::member::Member;
use data::Creature;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Debug)]
pub struct PartySwapEvent {
    pub from_position: usize,
    pub from_index: usize,
    pub to_position: usize,
    pub to_index: usize,
}

#[derive(PartialEq, Clone, Debug)]
struct PartyDragEvent {
    position: usize,
    index: usize,
}
impl PartyDragEvent {
    fn new(position: usize, index: usize) -> PartyDragEvent {
        PartyDragEvent { position, index }
    }
}

#[derive(Properties, PartialEq)]
pub struct PartyProps {
    pub party: Vec<Member>,
    pub on_swap: Callback<PartySwapEvent>,
}

#[function_component(Party)]
pub fn party(props: &PartyProps) -> Html {
    let dragging: UseStateHandle<Option<PartyDragEvent>> = use_state(|| None);
    html! {
        <div class="party">
            {props.party.iter().enumerate().map(|(i, m)| {
                let on_drag_start = {
                    let dragging = dragging.clone();
                    Callback::from(move |e: PartyDragEvent| {
                        log::debug!("dragstart: {:?}", e);
                        dragging.set(Some(e));
                    })
                };
                let on_drop = {
                    let dragging = dragging.clone();
                    let on_swap = props.on_swap.clone();
                    Callback::from(move |e: PartyDragEvent| {
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
    on_drag_start: Callback<PartyDragEvent>,
    on_drop: Callback<PartyDragEvent>,
}

#[function_component(PartyMember)]
fn party_member(props: &PartyMemberProps) -> Html {
    let on_drag_start = |index: usize| {
        let on_drag_start = props.on_drag_start.clone();
        let e = PartyDragEvent::new(props.position, index);
        Callback::from(move |_| {
            log::debug!("ondragstart");
            on_drag_start.emit(e.clone());
        })
    };
    let on_drop = |index: usize| {
        let on_drop = props.on_drop.clone();
        let e = PartyDragEvent::new(props.position, index);
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
                            creature={props.member.primary_creature.clone()}
                            empty_text={"Click to add primary trait"}
                            on_drag_start={on_drag_start(0).clone()}
                            on_drop={on_drop(0).clone()}
                        />
                    </li>
                    <li>
                        <PartyTrait
                            creature={props.member.fused_creature.clone()}
                            empty_text={"Click to add fused trait"}
                            on_drag_start={on_drag_start(1).clone()}
                            on_drop={on_drop(1).clone()}
                        />
                    </li>
                    <li>
                        <PartyTrait
                            creature={props.member.artifact_creature.clone()}
                            empty_text={"Click to add artifact trait"}
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
    creature: Option<Creature>,
    empty_text: &'static str,
    on_drag_start: Callback<()>,
    on_drop: Callback<()>,
}

#[function_component(PartyTrait)]
fn party_trait(props: &PartyTraitProps) -> Html {
    let opacity = use_state(|| 1.0);

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

    if let Some(c) = &props.creature {
        html! {
                <>
                    <div
                        class="trait non-empty"
                        style={format!("opacity: {}", opacity.deref())}
                        draggable="true"
                        ondragstart={ondragstart}
                        ondragend={ondragend}
                        ondragover={ondragover}
                        ondrop={ondrop}
                    >
                        <div class="creature">
                            <span>{c.creature.as_str()}</span>
                        </div>
                        <div class="trait-name">
                            <span>{c.trait_name.as_str()}</span>
                        </div>
                        <div class="trait-description">
                            <span>{c.trait_description.as_str()}</span>
                        </div>
                    </div>
                    <div class="clear">
        //                     Icon { width: 24, height: 24, icon: bs_icons::BsXLg }
                        <button></button>
                    </div>
                </>
            }
    } else {
        html! {
            <>
                <div class="trait empty"
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
