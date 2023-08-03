#![allow(non_snake_case)]

use crate::component::class_icon::ClassIcon;
use dioxus::prelude::*;

#[inline_props]
pub fn CreatureCard(cx: Scope) -> Element {
    render! {
        div {
            class: "card card-compact card-side bg-neutral text-neutral-content",
            figure {
                class: "py-2 px-4",
                img {
                    class: "w-24 h-24",
                    src: "battle_sprites/spr_crits_battle_2933.png",
                }
            }
            div {
                class: "card-body !p-0", // divide-y divide-base
                h2 {
                    class: "card-title leading-4 !mb-0",
                    span {
                        class: "badge badge-lg bg-black/25 text-neutral-content",
                        ClassIcon { class: "mr-1", name: "Death" }
                        "Death"
                    }
                    span {
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
                }
                dl {
                    // div {
                    //     class: "grid grid-cols-3 gap-4",
                    //     dt { class: "text-sm font-bold leading-6", "Family" }
                    //     dd { class: "text-sm col-span-2 leading-6", "Avatar" }
                    // }
                    div {
                        class: "grid grid-cols-3 gap-4",
                        dt { class: "text-sm font-bold leading-6", "Trait" }
                        dd { class: "text-sm col-span-2 leading-6", "Avatar" }
                    }
                    div {
                        class: "grid grid-cols-3 gap-4",
                        dt { class: "text-sm font-bold leading-6", "Material" }
                        dd { class: "text-sm col-span-2 leading-6", "N/A" }
                    }
                    div {
                        class: "grid grid-cols-3 gap-4",
                        dt { class: "text-sm font-bold leading-6", "Sources" }
                        dd {
                            class: "text-sm col-span-2 leading-6",
                            ul {
                                li { "Gate of the Gods" }
                                li { "Gate of the Gods" }
                            }
                        }
                    }

                }
            }
        }
    }
}
