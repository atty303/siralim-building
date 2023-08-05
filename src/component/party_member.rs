#![allow(non_snake_case)]

use data::r#trait::Trait;
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
) -> Element<'a> {
    render! {
        div {
            class: "card card-bordered border-base-300 card-side card-compact w-full shadow-sm shadow-black/50 bg-base-300",
            figure {
                class: "bg-nature/50 p-4 relative",
                img {
                    class: "inline-block w-28 h-28",
                    src: "battle_sprites/spr_crits_battle_2933.png",
                }
                div {
                    class: "badge absolute inset-x-2 bottom-2 text-center font-bold w-auto bg-black/25 text-sm h-8",
                    img {
                        class: "inline-block mr-1",
                        src: "images/nature.png"
                    }
                    "Nature"
                }
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
                            "Alexandria"
                        }
                        span {
                            class: "text-sm pl-2",
                            "Diabolic Horde"
                        }
                    }

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
                                "30"
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
                                "22"
                            }
                            span {
                                class: "py-1 px-2 bg-error text-error-content rounded-md",
                                img {
                                    class: "inline-block mr-2",
                                    src: "images/intelligence.png",
                                }
                                "26"
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
                                "18"
                            }
                            span {
                                img {
                                    class: "inline-block mr-2",
                                    src: "images/speed.png",
                                }
                                "28"
                            }
                        }
                    }

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
                        }
                        MemberTrait {
                            index: 1,
                            r#trait: member.traits[1],
                            empty_text: "Click to add a fused trait",
                            on_click: |i| on_trait_click.call(i),
                        }
                        MemberTrait {
                            index: 2,
                            r#trait: member.traits[2],
                            empty_text: "Click to add a artifact trait",
                            on_click: |i| on_trait_click.call(i),
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
                                class: "font-bold bg-secondary text-secondary-content p-2 w-48 rounded-md underline decoration-dotted",
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

#[derive(Props)]
struct MemberTraitProps<'a> {
    index: usize,
    #[props(!optional)]
    r#trait: Option<&'a Trait>,
    empty_text: &'static str,
    on_click: EventHandler<'a, usize>,
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
                    class: "font-bold bg-secondary text-secondary-content p-2 w-48 rounded-md underline decoration-dotted",
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
