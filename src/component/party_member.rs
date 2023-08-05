#![allow(non_snake_case)]

use data::r#trait::Trait;
use data::stat::Stat;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::component::card_tooltip::CardTooltip;
use crate::component::creature_card::CreatureCard;
use crate::component::outline_icon::OutlineIcon;
use crate::state::Member;

#[inline_props]
pub fn PartyMember<'a>(
    cx: Scope<'a>,
    member: Member<'a>,
    on_trait_click: EventHandler<'a, usize>,
    on_trait_clear: EventHandler<'a, usize>,
) -> Element<'a> {
    render! {
        div {
            class: "card card-bordered border-base-300 card-side card-compact w-full shadow-sm shadow-black/50 bg-base-300",
            MemberFigure {
                member: member,
            }

            div {
                class: "card-body",

                div {
                    class: "card-title flex items-center gap-4",
                    div {
                        class: "text-primary hover:text-primary-focus cursor-pointer",
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
                    div {
                        class: "[writing-mode:vertical-rl] text-center bg-secondary text-secondary-content rounded-md self-stretch rotate-180",
                        "TRAITS"
                    }

                    div {
                        class: "space-y-2 grow",
                        MemberTrait {
                            index: 0,
                            r#trait: member.traits[0],
                            empty_text: "Click to add a primary trait",
                            on_click: |i| on_trait_click.call(i),
                            on_clear: |i| on_trait_clear.call(i),
                        }
                        MemberTrait {
                            index: 1,
                            r#trait: member.traits[1],
                            empty_text: "Click to add a fused trait",
                            on_click: |i| on_trait_click.call(i),
                            on_clear: |i| on_trait_clear.call(i),
                        }
                        MemberTrait {
                            index: 2,
                            r#trait: member.traits[2],
                            empty_text: "Click to add a artifact trait",
                            on_click: |i| on_trait_click.call(i),
                            on_clear: |i| on_trait_clear.call(i),
                        }
                    }

                    div {
                        class: "[writing-mode:vertical-rl] text-center bg-secondary text-secondary-content rounded-md self-stretch rotate-180",
                        "SPELLS"
                    }

                    div {
                        class: "space-y-2 grow",

                        div {
                            class: "flex items-center p-2 gap-2 rounded-md bg-base-100",
                            div {
                                class: "text-primary hover:text-primary-focus cursor-pointer",
                                OutlineIcon {
                                    icon: Shape::Bars3,
                                }
                            }
                            div {
                                class: "font-bold bg-secondary text-secondary-content p-2 w-64 rounded-md underline decoration-dotted",
                                img {
                                    class: "inline-block mr-2",
                                    src: "images/chaos.png",
                                }
                                "Short Fuse"
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
                                OutlineIcon {
                                    icon: Shape::XMark,
                                    size: 16,
                                }
                            }
                        }

                        div {
                            class: "text-center p-2 rounded-md bg-base-100 text-primary cursor-pointer",
                            "Click to add a spell"
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn MemberFigure<'a>(cx: Scope<'a>, member: &'a Member<'a>) -> Element<'a> {
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
            class: "{class_color} p-4 relative",
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
                        class: "badge absolute inset-x-2 bottom-2 text-center font-bold w-auto bg-black/25 text-sm h-8",
                        img {
                            class: "inline-block mr-1",
                            src: "images/{c}.png"
                        }
                        "{c}"
                    }
                }
            }
        }

    }
}

#[inline_props]
fn MemberStats<'a>(cx: Scope<'a>, member: &'a Member<'a>) -> Element<'a> {
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
fn MemberArtifact<'a>(cx: Scope<'a>, _member: &'a Member<'a>) -> Element<'a> {
    render! {
        div {
            class: "flex items-center",
            button {
                class: "btn btn-primary btn-xs mr-2",
                "Artifacts"
            }
            span {
                class: "underline decoration-dotted whitespace-nowrap",
                img {
                    class: "inline-block mr-2",
                    src: "images/boots_0_0.png"
                }
                "Boots"
            }
        }
    }
}

#[inline_props]
fn MemberRelic<'a>(cx: Scope<'a>, _member: &'a Member<'a>) -> Element<'a> {
    render! {
        div {
            class: "flex items-center",
            button {
                class: "btn btn-primary btn-xs mr-2",
                "Relic"
            }
            span {
                class: "underline decoration-dotted whitespace-nowrap",
                "5740-NG"
            }
        }
    }
}

#[derive(Props)]
struct MemberTraitProps<'a> {
    index: usize,
    #[props(!optional)]
    r#trait: Option<&'a Trait>,
    empty_text: &'static str,
    on_click: EventHandler<'a, usize>,
    on_clear: EventHandler<'a, usize>,
}

fn MemberTrait<'a>(cx: Scope<'a, MemberTraitProps<'a>>) -> Element<'a> {
    if let Some(t) = &cx.props.r#trait {
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
                    class: "font-bold bg-secondary text-secondary-content p-2 w-64 rounded-md underline decoration-dotted",
                    CardTooltip {
                        tip: render! { CreatureCard { r#trait: *t } },
                        img {
                            class: "inline-block mr-2",
                            src: "images/death.png",
                        }
                        "{t.creature}"
                    }
                }
                div {
                    class: "grow",
                    "After a creature gains or loses stats, its allies gain or lose 15% of those stats as well. This trait does not stack."
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
    } else {
        render! {
            div {
                class: "text-center p-2 rounded-md bg-base-100 text-primary hover:text-primary-focus cursor-pointer",
                onclick: |_| cx.props.on_click.call(cx.props.index),
                cx.props.empty_text
            }
        }
    }
}
