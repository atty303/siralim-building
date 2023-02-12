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
                            td { class: "class", c.class.as_str() }
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
