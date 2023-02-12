#![allow(non_snake_case)]

use crate::data::Creature;
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct CreatureTableProps {
    items: Vec<Creature>,
}

impl CreatureTableProps {
    //    fn from()
}

pub fn CreatureTable(cx: Scope<CreatureTableProps>) -> Element {
    cx.render(rsx! {
        table {
            class: "creatures",
            thead {
                tr {
                    th { "Class" }
                    th { "Family" }
                    th { "Creature" }
                    th { "Trait" }
                    th { "Trait Description" }
                    th { "Material Name" }
                }
            }
            tbody {
                cx.props.items.iter().map(|c|
                    rsx! {
                        tr {
                            td { class: "class", CreatureClass { value: c.class.clone() } }
                            td { class: "family", c.family.as_str() }
                            td { class: "creature", c.creature.as_str() }
                            td { class: "trait", c.trait_name.as_str() }
                            td { class: "trait_description", c.trait_description.as_str() }
                            td { class: "material_name", c.material_name.as_str() }
                        }
                    }
                )
            }
        }
    })
}

#[derive(PartialEq, Props)]
pub struct CreatureClassProps {
    value: String,
}

fn CreatureClass(cx: Scope<CreatureClassProps>) -> Element {
    let lower_class = cx.props.value.to_lowercase();
    let icon = if ["nature", "chaos", "sorcery", "death", "life"].contains(&lower_class.as_str()) {
        rsx! { img { src: format_args!("image/{}.png", &lower_class) } }
    } else {
        rsx! { "" }
    };
    cx.render(rsx! { icon, cx.props.value.as_str() })
}
