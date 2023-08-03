#![allow(non_snake_case)]

use crate::component::card_tooltip::CardTooltip;
use crate::component::creature_card::CreatureCard;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::component::outline_icon::OutlineIcon;

pub fn PartyMember(cx: Scope) -> Element {
    render! {
        div {
            class: "card card-bordered border-neutral card-side card-compact w-full shadow bg-base-300",
            figure {
                class: "bg-nature/50 p-4 relative",
                img {
                    class: "inline-block",
                    src: "battle_sprites/spr_crits_battle_2933.png",
                }
                div {
                    class: "badge badge absolute inset-x-2 bottom-2 text-center font-bold w-auto bg-black/25",
                    img {
                        class: "inline-block",
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
                        class: "space-y-2",

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
                                    tip: render! { CreatureCard {} },
                                    img {
                                        class: "inline-block mr-2",
                                        src: "images/death.png",
                                    }
                                    "Alexandria"
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

                        div {
                            class: "text-center p-2 rounded-md bg-base-100 text-primary cursor-pointer",
                            "Click to add a trait"
                        }

                        div {
                            class: "text-center p-2 rounded-md bg-base-100 text-primary cursor-pointer",
                            "Click to add a trait"
                        }

                    }

                    div {
                        class: "[writing-mode:vertical-rl] text-center bg-secondary text-secondary-content rounded-md self-stretch rotate-180",
                        "SPELLS"
                    }

                    div {
                        class: "space-y-2",

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
