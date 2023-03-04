use data::spell::Spell;
use yew::prelude::*;

use crate::components::icon::ClassIcon;
use crate::components::tooltip::Tooltip;

#[derive(Properties, PartialEq)]
pub struct SpellNameProps {
    pub spell: Spell,
    pub icon: bool,
}

#[function_component(SpellName)]
pub fn spell_name(props: &SpellNameProps) -> Html {
    let c = &props.spell;

    let tooltip = html! {
        <div>
            <span class="charges"><span>{"Charges: "}</span>{format!("{}", c.charges)}</span><br/>
            <span class="sources"><span>{"Source:"}</span>{c.source.as_str()}</span>
        </div>
    };

    html! {
        <Tooltip {tooltip}>
            <span class="name-plate">
                {if props.icon {
                    html! { <ClassIcon value={c.class.clone()} /> }
                } else {
                    html! {}
                }}
                {c.name.as_str()}
            </span>
        </Tooltip>
    }
}
