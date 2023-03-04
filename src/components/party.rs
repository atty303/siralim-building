use web_sys::Element;

use data::personality::Stat;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::Dispatch;

use data::r#trait::Trait;
use data::spell::Spell;

use crate::components::creature_name::CreatureName;
use crate::components::description::Description;
use crate::components::icon::ClassIcon;
use crate::components::spell_name::SpellName;
use crate::state::{Action, Member, State};

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

#[derive(PartialEq, Clone, Debug)]
pub struct PartySpellEvent {
    pub position: usize,
    pub index: usize,
}
impl PartySpellEvent {
    fn new(position: usize, index: usize) -> PartySpellEvent {
        Self { position, index }
    }
}

#[derive(Properties, PartialEq)]
pub struct PartyProps {
    pub party: Vec<Member>,
    pub pool: Vec<Option<Trait>>,
    pub on_swap: Callback<PartySwapEvent>,
    pub on_click: Callback<PartyTraitEvent>,
    pub on_clear: Callback<PartyTraitEvent>,
    pub on_spell_click: Callback<PartySpellEvent>,
}

#[function_component(Party)]
pub fn party(props: &PartyProps) -> Html {
    let dragging: UseStateHandle<Option<PartyTraitEvent>> = use_state(|| None);
    let pool_position = props.party.len();
    html! {
        <>
        <div class="party">
            <h2>{"PARTY"}</h2>
            {props.party.iter().enumerate().map(|(i, m)| {
                let dispatch = Dispatch::<State>::new();
                let on_click = {
                    let on_click = props.on_click.clone();
                    Callback::from(move |e: PartyTraitEvent| {
                        on_click.emit(e.clone());
                    })
                };
                let on_clear = {
                    let on_clear = props.on_clear.clone();
                    Callback::from(move |e: PartyTraitEvent| {
                        on_clear.emit(e.clone());
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
                let on_spell_click = {
                    let on_spell_click = props.on_spell_click.clone();
                    Callback::from(move |e: PartySpellEvent| {
                        on_spell_click.emit(e.clone());
                    })
                };
                let on_spell_clear = dispatch.apply_callback(|e: PartySpellEvent| Action::ClearSpell((e.position, e.index)));
                html! {
                    <PartyMember
                        position={i}
                        member={m.clone()}
                        on_click={on_click}
                        on_clear={on_clear}
                        on_drag_start={on_drag_start}
                        on_drop={on_drop}
                        {on_spell_click}
                        {on_spell_clear}
                    >
                    </PartyMember>
                }
            }).collect::<Html>()}
        </div>
        <div class="party-pool">
            <h2>{"POOL"}</h2>
            <ul class="traits">
                {props.pool.iter().enumerate().map(|(i, m)| {
                    let on_click = {
                        let on_click = props.on_click.clone();
                        let e = PartyTraitEvent::new(pool_position, i);
                        Callback::from(move |_| on_click.emit(e.clone()))
                    };
                    let on_clear = {
                        let on_clear = props.on_clear.clone();
                        let e = PartyTraitEvent::new(pool_position, i);
                        Callback::from(move |_| on_clear.emit(e.clone()))
                    };
                    let on_drag_start = {
                        let dragging = dragging.clone();
                        let e = PartyTraitEvent::new(pool_position, i);
                        Callback::from(move |_|  dragging.set(Some(e.clone())))
                    };
                    let on_drop = {
                        let dragging = dragging.clone();
                        let on_swap = props.on_swap.clone();
                        let e = PartyTraitEvent::new(pool_position, i);
                        Callback::from(move |_| {
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
                        <li>
                            <PartyTrait
                                r#trait={m.clone()}
                                empty_text={"Click to add trait"}
                                on_click={on_click.clone()}
                                on_clear={on_clear.clone()}
                                on_drag_start={on_drag_start.clone()}
                                on_drop={on_drop.clone()}
                            />
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct PartyMemberProps {
    position: usize,
    member: Member,
    on_click: Callback<PartyTraitEvent>,
    on_clear: Callback<PartyTraitEvent>,
    on_drag_start: Callback<PartyTraitEvent>,
    on_drop: Callback<PartyTraitEvent>,
    on_spell_click: Callback<PartySpellEvent>,
    on_spell_clear: Callback<PartySpellEvent>,
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
    let on_clear = |index: usize| {
        let on_clear = props.on_clear.clone();
        let e = PartyTraitEvent::new(props.position, index);
        Callback::from(move |_| on_clear.emit(e.clone()))
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

    let sprite = props
        .member
        .primary_trait
        .clone()
        .map(|t| t.sprite)
        .flatten();

    let mut left_pane_classes = classes!("left-pane");
    if let Some(c) = props.member.class() {
        left_pane_classes.push(c.as_str().to_lowercase().to_string());
    }

    let dispatch = Dispatch::<State>::new();

    let set_personality = {
        let position = props.position;
        dispatch.apply_callback(move |x: (Stat, bool)| Action::SetPersonality((position, x.0, x.1)))
    };

    let on_spell_click = |index: usize| {
        let on_spell_click = props.on_spell_click.clone();
        let e = PartySpellEvent::new(props.position, index);
        Callback::from(move |_| {
            on_spell_click.emit(e.clone());
        })
    };
    let on_spell_clear = |index: usize| {
        let on_spell_clear = props.on_spell_clear.clone();
        let e = PartySpellEvent::new(props.position, index);
        Callback::from(move |_| on_spell_clear.emit(e.clone()))
    };

    html! {
        <div class="party-member">
            <div class={left_pane_classes}>
                <div class="sprite-stat">
                    {if let Some(sprite) = &sprite {
                        html! { <img class="sprite" src={format!("battle_sprites/{}", sprite)} /> }
                    } else {
                        html! { <div class="sprite"></div> }
                    }}
                    <ul class="stats">
                        <PartyStat stat={Stat::Health} value={props.member.health()} personality={props.member.personality_for(Stat::Health)} set_personality={set_personality.clone()} />
                        <PartyStat stat={Stat::Attack} value={props.member.attack()} personality={props.member.personality_for(Stat::Attack)} set_personality={set_personality.clone()} />
                        <PartyStat stat={Stat::Intelligence} value={props.member.intelligence()} personality={props.member.personality_for(Stat::Intelligence)} set_personality={set_personality.clone()} />
                        <PartyStat stat={Stat::Defense} value={props.member.defense()} personality={props.member.personality_for(Stat::Defense)} set_personality={set_personality.clone()} />
                        <PartyStat stat={Stat::Speed} value={props.member.speed()} personality={props.member.personality_for(Stat::Speed)} set_personality={set_personality.clone()} />
                    </ul>
                </div>
                <div class="class">
                    {if let Some(x) = props.member.class() {
                        html! {
                            <span>
                                <ClassIcon value={x.clone()} />
                                {x.as_str()}
                            </span>
                        }
                    } else {
                        html! { }
                    }}
                </div>
            </div>
            <div class="right-pane">
                <div class="member-group">
                    <div class="member-group__title">{"TRAITS"}</div>
                    <ul class="member-group__main party-item-container">
                        <li class="party-item-container__item">
                            <PartyTrait
                                r#trait={props.member.primary_trait.clone()}
                                empty_text={"Click to add primary trait"}
                                on_click={on_click(0).clone()}
                                on_clear={on_clear(0).clone()}
                                on_drag_start={on_drag_start(0).clone()}
                                on_drop={on_drop(0).clone()}
                            />
                        </li>
                        <li class="party-item-container__item">
                            <PartyTrait
                                r#trait={props.member.fused_trait.clone()}
                                empty_text={"Click to add fused trait"}
                                on_click={on_click(1).clone()}
                                on_clear={on_clear(1).clone()}
                                on_drag_start={on_drag_start(1).clone()}
                                on_drop={on_drop(1).clone()}
                            />
                        </li>
                        <li class="party-item-container__item">
                            <PartyTrait
                                r#trait={props.member.artifact_trait.clone()}
                                empty_text={"Click to add artifact trait"}
                                on_click={on_click(2).clone()}
                                on_clear={on_clear(2).clone()}
                                on_drag_start={on_drag_start(2).clone()}
                                on_drop={on_drop(2).clone()}
                            />
                        </li>
                    </ul>
                </div>
                <div class="member-group">
                    <div class="member-group__title">{"SPELLS"}</div>
                    <ul class="member-group__main party-item-container">
                         {props.member.spells.iter().enumerate().map(|(i, s)| {
                             html! {
                                 <li class="party-item-container__item">
                                     <PartySpell
                                          spell={s.clone()}
                                          on_click={on_spell_click(i).clone()}
                                          on_clear={on_spell_clear(i).clone()}
                                     />
                                 </li>
                             }
                         }).collect::<Html>()}
                         <li class="party-item-container__item">
                             <PartySpell
                                  spell={None}
                                  on_click={on_spell_click(props.member.spells.len()).clone()}
                                  on_clear={on_spell_clear(props.member.spells.len()).clone()}
                             />
                         </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct PartyTraitProps {
    r#trait: Option<Trait>,
    empty_text: &'static str,
    on_click: Callback<()>,
    on_clear: Callback<()>,
    on_drag_start: Callback<()>,
    on_drop: Callback<()>,
}

#[function_component(PartyTrait)]
fn party_trait(props: &PartyTraitProps) -> Html {
    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    let onclear = {
        let on_clear = props.on_clear.clone();
        Callback::from(move |_| on_clear.emit(()))
    };

    let ondragstart = {
        let on_drag_start = props.on_drag_start.clone();
        Callback::from(move |e: MouseEvent| {
            on_drag_start.emit(());
            let el: Element = e.target_unchecked_into::<Element>();
            log::debug!("{:?}", el.parent_element().unwrap().parent_element());
            el.parent_element()
                .unwrap()
                .parent_element()
                .unwrap()
                .set_attribute("draggable", "true")
                .unwrap();
        })
    };
    let onmouseup = {
        Callback::from(move |e: MouseEvent| {
            let el: Element = e.target_unchecked_into::<Element>();
            el.parent_element()
                .unwrap()
                .parent_element()
                .unwrap()
                .set_attribute("draggable", "false")
                .unwrap();
        })
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
            <div class="party-trait">
                <div
                    class="party-trait__main party-trait__main--non-empty"
                    {ondragover}
                    {ondrop}
                >
                    <div class="party-trait__handle" onmousedown={ondragstart} onmouseup={onmouseup}>
                        <Icon icon_id={IconId::BootstrapGripVertical} />
                    </div>
                    <div class="party-trait__name">
                        <CreatureName r#trait={c.clone()} icon={true} />
                    </div>
                    <div class="party-trait__description">
                        <Description value={c.trait_description.to_vec()} />
                    </div>
                </div>
                <div class="party-trait__clear">
                    <button onclick={onclear}><Icon icon_id={IconId::BootstrapXLg} /></button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="party-trait">
                <div
                    class="party-trait__main party-trait__main--empty"
                    onclick={onclick}
                    ondragover={ondragover}
                    ondrop={ondrop}
                >
                    <span>{props.empty_text}</span>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PartyStatProps {
    stat: Stat,
    value: Option<u8>,
    personality: Option<bool>,
    set_personality: Callback<(Stat, bool)>,
}

#[function_component(PartyStat)]
fn party_stat(props: &PartyStatProps) -> Html {
    let format_stat = |v: Option<u8>| v.map_or_else(|| "-".to_string(), |n| format!("{}", n));
    let on_positive = {
        let set_personality = props.set_personality.clone();
        let stat = props.stat.clone();
        Callback::from(move |_| {
            set_personality.emit((stat.clone(), true));
        })
    };
    let on_negative = {
        let set_personality = props.set_personality.clone();
        let stat = props.stat.clone();
        Callback::from(move |_| set_personality.emit((stat.clone(), false)))
    };
    html! {
        <li class={classes!(props.stat.to_string(), "tooltip")}>
            <span class="tooltip-text">
                <span>{"Personality: "}<img src={format!("image/{}.png", props.stat)} /></span>
                <button onclick={on_positive}><Icon icon_id={IconId::BootstrapArrowUpShort} /></button>
                <button onclick={on_negative}><Icon icon_id={IconId::BootstrapArrowDownShort} /></button>
            </span>
            <img src={format!("image/{}.png", props.stat)} />
            {format_stat(props.value)}
            {if props.personality == Some(true) {
                html! { <Icon icon_id={IconId::BootstrapArrowUpShort} class="personality positive" /> }
            } else {
                html! {}
            }}
            {if props.personality == Some(false) {
                html! { <Icon icon_id={IconId::BootstrapArrowDownShort} class="personality negative" /> }
            } else {
                html! {}
            }}
        </li>
    }
}

#[derive(Properties, PartialEq)]
pub struct PartySpellProps {
    spell: Option<Spell>,
    on_click: Callback<()>,
    on_clear: Callback<()>,
}

#[function_component(PartySpell)]
fn party_spell(props: &PartySpellProps) -> Html {
    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };
    let onclear = {
        let on_clear = props.on_clear.clone();
        Callback::from(move |_| on_clear.emit(()))
    };
    if let Some(c) = &props.spell {
        html! {
            <div class="party-spell">
                <div class="party-spell__spell party-spell__spell--non-empty">
                    <div class="party-spell__name">
                        <SpellName spell={c.clone()} icon={true} />
                    </div>
                    <div class="party-spell__description">
                        <Description value={c.description.to_vec()} />
                    </div>
                </div>
                <SpellProperty />
                <SpellProperty />
                <SpellProperty />
                <div class="party-spell__clear">
                    <button onclick={onclear}><Icon icon_id={IconId::BootstrapXLg} /></button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="party-spell">
                <div class="party-spell__spell party-spell__spell--empty" {onclick}>
                    <span>{"Click to add spell"}</span>
                </div>
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SpellPropertyProps {}

#[function_component(SpellProperty)]
fn spell_property(_props: &SpellPropertyProps) -> Html {
    html! {
        <div class="party-spell__property">
            <button>
                <Icon icon_id={IconId::BootstrapPlusCircleDotted} />
            </button>
        </div>
    }
}
