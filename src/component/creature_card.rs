#![allow(non_snake_case)]

use crate::component::class_icon::ClassIcon;
use data::r#trait::Trait;
use dioxus::prelude::*;

#[inline_props]
pub fn CreatureCard<'a>(cx: Scope<'a>, r#trait: &'a Trait) -> Element<'a> {
    let t = r#trait;

    render! {
        div {
            class: "card card-compact card-side bg-neutral text-neutral-content",
            if let Some(sprite) = t.sprite.as_ref() {
                rsx! {
                    figure {
                        class: "py-2 px-4",
                        img {
                            class: "w-24 h-24",
                            src: "battle_sprites/{sprite}",
                        }
                    }
                }
            }
            div {
                class: "card-body !p-0", // divide-y divide-base
                h2 {
                    class: "card-title leading-4 !mb-0 whitespace-nowrap",
                    span {
                        class: "badge badge-lg bg-black/25 text-neutral-content",
                        ClassIcon { class: "mr-1", name: t.class.as_str() }
                        "{t.class}"
                    }
                    span {
                        class: "grow divide-x divide-base",
                        span {
                            class: "pr-2",
                            "{t.creature}"
                        }
                        span {
                            class: "text-sm pl-2",
                            "{t.family}"
                        }
                    }
                }
                if let Some(s) = t.stats.as_ref() {
                    rsx! {
                        div {
                            class: "text-sm font-normal col-span-2 leading-6 space-x-2 whitespace-nowrap",
                            span {
                                img {
                                    class: "inline-block w-4 h-4 mr-1",
                                    src: "images/health.png",
                                }
                                "{s.health}"
                            }
                            span {
                                img {
                                    class: "inline-block w-4 h-4 mr-1",
                                    src: "images/attack.png",
                                }
                                "{s.attack}"
                            }
                            span {
                                img {
                                    class: "inline-block w-4 h-4 mr-1",
                                    src: "images/intelligence.png",
                                }
                                "{s.intelligence}"
                            }
                            span {
                                img {
                                    class: "inline-block w-4 h-4 mr-1",
                                    src: "images/defense.png",
                                }
                                "{s.defense}"
                            }
                            span {
                                img {
                                    class: "inline-block w-4 h-4 mr-1",
                                    src: "images/speed.png",
                                }
                                "{s.speed}"
                            }
                        }
                    }
                }
                dl {
                    div {
                        class: "grid grid-cols-3 gap-4",
                        dt { class: "text-sm font-bold leading-6", "Trait" }
                        dd {
                            class: "text-sm font-normal col-span-2 leading-6",
                            "{t.trait_name}"
                        }
                    }
                    div {
                        class: "grid grid-cols-3 gap-4",
                        dt { class: "text-sm font-bold leading-6", "Material" }
                        dd {
                            class: "text-sm font-normal col-span-2 leading-6",
                            "{t.material_name}"
                        }
                    }
                    if !t.sources.is_empty() {
                        rsx! {
                            div {
                                class: "grid grid-cols-3 gap-4",
                                dt { class: "text-sm font-bold leading-6", "Sources" }
                                dd {
                                    class: "text-sm font-normal col-span-2 leading-6",
                                    ul {
                                        for s in t.sources.iter() {
                                            li { "{s}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
