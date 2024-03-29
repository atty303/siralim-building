#![allow(non_snake_case)]

use dioxus::prelude::*;
// TODO(PR): outline の場合は stroke-width, stroke: "currentColor" を指定できるようにする
use dioxus_heroicons::{Icon, IconShape};

#[inline_props]
pub fn OutlineIcon<S: IconShape>(cx: Scope, icon: S, size: Option<u32>) -> Element {
    render! {
        Icon {
            class: "stroke-current stroke-2 inline-block",
            icon: icon.clone(),
            size: size.unwrap_or(24),
            fill: "none",
        }
    }
}
