use yew::prelude::*;

use data::r#trait::Trait;

use crate::components::icon::ClassIcon;
use crate::components::tooltip::Tooltip;

#[derive(Properties, PartialEq)]
pub struct CreatureNameProps {
    pub r#trait: Trait,
    pub icon: bool,
}

#[function_component(CreatureName)]
pub fn creature_name(props: &CreatureNameProps) -> Html {
    let c = &props.r#trait;

    let tooltip = html! {
        <div class="creature-name-tooltip">
            {if let Some(sprite) = &c.sprite {
                html! { <img class="sprite" src={format!("battle_sprites/{}", sprite)} /> }
            } else {
                html! { }
            }}
            {if let Some(stats) = &c.stats {
                html! {
                    <ul class="creature-name-tooltip__stats">
                        <li class="health"><img src="image/health.png" />{format!("{}", stats.health)}</li>
                        <li class="attack"><img src="image/attack.png" />{format!("{}", stats.attack)}</li>
                        <li class="intelligence"><img src="image/intelligence.png" />{format!("{}", stats.intelligence)}</li>
                        <li class="defense"><img src="image/defense.png" />{format!("{}", stats.defense)}</li>
                        <li class="speed"><img src="image/speed.png" />{format!("{}", stats.speed)}</li>
                    </ul>
                }
            } else {
                html! { }
            }}
            <div>
                <span class="creature-name-tooltip__creature"><ClassIcon value={c.class.clone()} />{c.class.as_str()}{": "}{c.creature.as_str()}</span><br/>
                <span class="creature-name-tooltip__family"><span>{"Family: "}</span>{c.family.as_str()}</span><br/>
                <span class="creature-name-tooltip__trait"><span>{"Trait: "}</span>{c.trait_name.as_str()}</span><br/>
                <span class="creature-name-tooltip__material"><span>{"Material: "}</span>{c.material_name.as_str()}</span><br/>
                <span class="creature-name-tooltip__sources"><span>{"Sources:"}</span>
                    <ul>
                        {c.sources.iter().map(|s| html! { <li>{s}</li> }).collect::<Html>()}
                    </ul>
                </span>
            </div>
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
                {c.creature.as_str()}
            </span>
        </Tooltip>
    }
}
