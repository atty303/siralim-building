#![allow(non_snake_case)]

use classes::classes;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::component::app::{MemberDndContext, TraitDndContext};
use data::r#trait::Trait;
use data::stat::Stat;

use crate::component::card_tooltip::CardTooltip;
use crate::component::class_icon::ClassIcon;
use crate::component::creature_card::CreatureCard;
use crate::component::description::Description;
use crate::component::outline_icon::OutlineIcon;
use crate::hooks::drag::{use_draggable, use_droppable};
use crate::hooks::persistent::UsePersistent;
use crate::state::Member;

#[inline_props]
pub fn PartyMember<'a>(
    cx: Scope<'a>,
    index: usize,
    member: Member,
    on_trait_click: EventHandler<'a, usize>,
    on_trait_clear: EventHandler<'a, usize>,
    show_traits: UsePersistent<bool>,
    show_spells: UsePersistent<bool>,
) -> Element<'a> {
    let c = MemberDndContext { index: *index };
    let id = c.to_id();
    let draggable = use_draggable::<MemberDndContext>(cx, id.clone());
    let droppable = use_droppable::<MemberDndContext>(cx, id.clone());

    let x = render! {
        div {
            class: "card card-bordered border-base-300 card-side card-compact w-full shadow-sm shadow-black/50 bg-base-300",
            prevent_default: "ondragover ondrop",
            draggable: *draggable.draggable.read(),
            onmounted: move |e| draggable.onmounted.call(e),
            onmousedown: move |e| draggable.onmousedown.call(e),
            ondragstart: move |e| draggable.ondragstart.call(e),
            ondragend: move |e| draggable.ondragend.call(e),
            ondragover: move |e| droppable.ondragover.call(e),
            ondrop: move |e| droppable.ondrop.call(e),

            MemberFigure {
                member: member,
            }

            div {
                class: "card-body",

                div {
                    class: "card-title flex items-center gap-4",
                    div {
                        class: "text-primary hover:text-primary-focus cursor-pointer",
                        onmounted: move |e| draggable.activator.onmounted.call(e),
                        onmousedown: move |e| draggable.activator.onmousedown.call(e),
                        OutlineIcon {
                            icon: Shape::Bars3,
                        }
                    }

                    div {
                        class: "grow divide-x divide-base",
                        span {
                            class: "pr-2",
                            "{member.creature().unwrap_or_default()}"
                        }
                        span {
                            class: "text-sm pl-2",
                            "{member.family().unwrap_or_default()}"
                        }
                    }

                    MemberStats {
                        member: member,
                    }
                    MemberArtifact {
                        _member: member,
                    }
                    MemberRelic {
                        _member: member,
                    }
                }

                div {
                    class: "flex items-start gap-2",

                    MemberCollapsable {
                        state: show_traits.clone(),
                        text: "TRAITS",

                        MemberTrait {
                            index: 0,
                            member_index: *index,
                            r#trait: member.traits[0],
                            empty_text: "Click to add a primary trait",
                            on_click: |i| on_trait_click.call(i),
                            on_clear: |i| on_trait_clear.call(i),
                        }
                        MemberTrait {
                            index: 1,
                            member_index: *index,
                            r#trait: member.traits[1],
                            empty_text: "Click to add a fused trait",
                            on_click: |i| on_trait_click.call(i),
                            on_clear: |i| on_trait_clear.call(i),
                        }
                        MemberTrait {
                            index: 2,
                            member_index: *index,
                            r#trait: member.traits[2],
                            empty_text: "Click to add a artifact trait",
                            on_click: |i| on_trait_click.call(i),
                            on_clear: |i| on_trait_clear.call(i),
                        }
                    }

                    MemberCollapsable {
                        state: show_spells.clone(),
                        text: "SPELLS",

                        MemberSpell {
                            index: 0,
                            spell: None,
                            on_click: |_i| {},
                            on_clear: |_i| {},
                        }
                    }
                }
            }
        }
    };
    x
}

#[inline_props]
fn MemberFigure<'a>(cx: Scope<'a>, member: &'a Member) -> Element<'a> {
    let mc = member.class();
    let class_name = mc.as_ref().map(|c| c.as_str());
    let class_color = match class_name {
        Some("Death") => "bg-death/50",
        Some("Nature") => "bg-nature/50",
        Some("Life") => "bg-life/50",
        Some("Sorcery") => "bg-sorcery/50",
        Some("Chaos") => "bg-chaos/50",
        _ => "bg-base/50",
    };

    render! {
        figure {
            class: "{class_color} p-8 relative w-64 max-w-max",
            if let Some(sprite) = member.sprite() {
                rsx! {
                    img {
                        class: "block w-28 h-28",
                        src: "battle_sprites/{sprite}",
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "w-28 h-28",
                    }
                }
            }
            if let Some(c) = class_name {
                rsx! {
                    div {
                        class: "badge absolute inset-x-2 bottom-2 text-center font-bold w-auto !bg-base-100/50 text-sm h-8",
                        ClassIcon {
                            class: "mr-1",
                            name: "{c}"
                        }
                        "{c}"
                    }
                }
            }
        }

    }
}

#[inline_props]
fn MemberStats<'a>(cx: Scope<'a>, member: &'a Member) -> Element<'a> {
    let format_stat = |stat: Stat| -> String {
        member
            .stats(stat)
            .map_or("-".to_string(), |s| format!("{}", s))
    };

    render! {
        div {
            class: "flex items-center whitespace-nowrap",
            button {
                class: "btn btn-primary btn-xs mr-2",
                "Stats"
            }
            span {
                class: "space-x-4 text-sm",

                span {
                    class: "py-1 px-2 bg-success text-success-content rounded-md",
                    img {
                        class: "inline-block mr-2",
                        src: "images/health.png",
                    }
                    format_stat(Stat::Health)
                    OutlineIcon {
                        icon: Shape::ArrowUp,
                        size: 16,
                    }
                }
                span {
                    img {
                        class: "inline-block mr-2",
                        src: "images/attack.png",
                    }
                    format_stat(Stat::Attack)
                }
                span {
                    class: "py-1 px-2 bg-error text-error-content rounded-md",
                    img {
                        class: "inline-block mr-2",
                        src: "images/intelligence.png",
                    }
                    format_stat(Stat::Intelligence)
                    OutlineIcon {
                        icon: Shape::ArrowDown,
                        size: 16,
                    }
                }
                span {
                    img {
                        class: "inline-block mr-2",
                        src: "images/defense.png",
                    }
                    format_stat(Stat::Defense)
                }
                span {
                    img {
                        class: "inline-block mr-2",
                        src: "images/speed.png",
                    }
                    format_stat(Stat::Speed)
                }
            }
        }
    }
}

#[inline_props]
fn MemberArtifact<'a>(cx: Scope<'a>, _member: &'a Member) -> Element<'a> {
    render! {
        div {
            class: "flex items-center",
            button {
                class: "btn btn-primary btn-xs mr-2",
                "Artifacts"
            }
            if false {
                rsx! {
                    span {
                        class: "underline decoration-dotted whitespace-nowrap",
                        img {
                            class: "inline-block mr-2",
                            src: "images/boots_0_0.png"
                        }
                        "Boots"
                    }
                }
            } else {
                rsx! { "-" }
            }
        }
    }
}

#[inline_props]
fn MemberRelic<'a>(cx: Scope<'a>, _member: &'a Member) -> Element<'a> {
    render! {
        div {
            class: "flex items-center",
            button {
                class: "btn btn-primary btn-xs mr-2",
                "Relic"
            }
            if false {
                rsx! {
                    span {
                        class: "underline decoration-dotted whitespace-nowrap",
                        "5740-NG"
                    }
                }
            } else {
                rsx! { "-" }
            }
        }
    }
}

#[inline_props]
fn MemberCollapsable<'a>(
    cx: Scope<'a>,
    state: UsePersistent<bool>,
    text: &'a str,
    children: Element<'a>,
) -> Element<'a> {
    let class = classes!["space-y-2 grow", "hidden" => !state.get()];
    let icon = if state.get() {
        Shape::ChevronRight
    } else {
        Shape::ChevronLeft
    };

    render! {
        button {
            class: "[writing-mode:vertical-rl] bg-primary text-primary-content rounded-md self-stretch rotate-180 flex items-center justify-center px-1 py-8",
            onclick: |_| state.set(!state.get()),
            OutlineIcon { icon: icon, size: 20, }
            span {
                class: "my-2",
                *text
            }
            OutlineIcon { icon: icon, size: 20, }
        }
        div {
            class: "{class}",
            children
        }
    }
}

#[derive(Props)]
struct MemberTraitProps<'a> {
    index: usize,
    member_index: usize,
    #[props(!optional)]
    r#trait: Option<&'a Trait>,
    empty_text: &'static str,
    on_click: EventHandler<'a, usize>,
    on_clear: EventHandler<'a, usize>,
}

fn MemberTrait<'a>(cx: Scope<'a, MemberTraitProps<'a>>) -> Element<'a> {
    let c = TraitDndContext {
        member_index: cx.props.member_index,
        trait_index: cx.props.index,
    };
    let id = c.to_id();
    let draggable = use_draggable::<TraitDndContext>(cx, id.clone());
    let droppable = use_droppable::<TraitDndContext>(cx, id.clone());

    if let Some(t) = cx.props.r#trait {
        render! {
            div {
                prevent_default: "ondragover ondrop",
                draggable: *draggable.draggable.read(),
                onmounted: move |e| draggable.onmounted.call(e),
                onmousedown: move |e| draggable.onmousedown.call(e),
                ondragstart: move |e| draggable.ondragstart.call(e),
                ondragend: move |e| draggable.ondragend.call(e),
                ondragover: move |e| droppable.ondragover.call(e),
                ondrop: move |e| droppable.ondrop.call(e),
                div {
                    class: "flex items-center p-2 gap-2 rounded-md bg-base-100",
                    div {
                        onmounted: move |e| draggable.activator.onmounted.call(e),
                        onmousedown: move |e| draggable.activator.onmousedown.call(e),
                        class: "text-primary hover:text-primary-focus cursor-pointer",
                        OutlineIcon {
                            icon: Shape::Bars3,
                        }
                    }
                    div {
                        class: "bg-secondary text-secondary-content py-2 px-4 min-w-max rounded-md ",
                        CardTooltip {
                            tip: render! { CreatureCard { r#trait: t } },
                            class: "font-bold underline decoration-dotted",
                            ClassIcon {
                                class: "inline-block mr-2",
                                name: "{t.class}",
                            }
                            "{t.creature}"
                        }
                    }
                    div {
                        class: "grow",
                        Description {
                            value: t.trait_description.clone(),
                        }
                    }
                    button {
                        class: "btn btn-primary btn-circle btn-xs",
                        onclick: move |_| cx.props.on_clear.call(cx.props.index),
                        OutlineIcon {
                            icon: Shape::XMark,
                            size: 16,
                        }
                    }
                }
            }
        }
    } else {
        render! {
            div {
                prevent_default: "ondragover ondrop",
                class: "text-center p-4 rounded-md bg-base-100 text-primary hover:text-primary-focus cursor-pointer font-bold whitespace-nowrap",
                ondragover: move |e| droppable.ondragover.call(e),
                ondrop: move |e| droppable.ondrop.call(e),
                onclick: |_| cx.props.on_click.call(cx.props.index),
                cx.props.empty_text
            }
        }
    }
}

#[derive(Props)]
struct MemberSpellProps<'a> {
    index: usize,
    #[props(!optional)]
    spell: Option<&'a Trait>,
    on_click: EventHandler<'a, usize>,
    on_clear: EventHandler<'a, usize>,
}

fn MemberSpell<'a>(cx: Scope<'a, MemberSpellProps<'a>>) -> Element<'a> {
    if let Some(_s) = cx.props.spell {
        render! {
            div {
                class: "flex items-center p-2 gap-2 rounded-md bg-base-100",
                div {
                    class: "text-primary hover:text-primary-focus cursor-pointer",
                    OutlineIcon {
                        icon: Shape::Bars3,
                    }
                }
                div {
                    class: "bg-secondary text-secondary-content py-2 px-4 min-w-max rounded-md",
                    CardTooltip {
                        tip: render! { "placeholder" },
                        class: "font-bold underline decoration-dotted",
                        img {
                            class: "inline-block mr-2",
                            src: "images/chaos.png",
                        }
                        "Short Fuse"
                    }
                }
                div {
                    class: "grow",
                    "Enemies take damage equal to 100% of the potency of their Bomb debuffs."
                }
                button {
                    class: "btn btn-primary btn-circle btn-xs",
                    OutlineIcon {
                        icon: Shape::Cube,
                        size: 16,
                    }
                }
                button {
                    class: "btn btn-primary btn-circle btn-xs ",
                    onclick: move |_| cx.props.on_clear.call(cx.props.index),
                    OutlineIcon {
                        icon: Shape::XMark,
                        size: 16,
                    }
                }
            }
        }
    } else {
        render! {
            div {
                class: "text-center p-4 rounded-md bg-base-100 text-primary hover:text-primary-focus cursor-pointer font-bold whitespace-nowrap",
                onclick: |_| cx.props.on_click.call(cx.props.index),
                "Click to add a spell"
            }
        }
    }
}
