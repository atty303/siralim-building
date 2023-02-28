use yew::prelude::*;

use data::r#trait::Trait;

use crate::components::icon::ClassIcon;

#[derive(Properties, PartialEq)]
pub struct CreatureNameProps {
    pub r#trait: Trait,
    pub icon: bool,
}

#[function_component(CreatureName)]
pub fn creature_name(props: &CreatureNameProps) -> Html {
    let c = &props.r#trait;
    html! {
        <div class="creature-name tooltip">
            <div class="tooltip-text">
                {if let Some(sprite) = &c.sprite {
                    html! { <img class="sprite" src={format!("battle_sprites/{}", sprite)} /> }
                } else {
                    html! { }
                }}
                {if let Some(stats) = &c.stats {
                    html! {
                        <ul class="stats">
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
                    <span class="creature"><ClassIcon value={c.class.clone()} />{c.class.as_str()}{": "}{c.creature.as_str()}</span><br/>
                    <span class="family"><span>{"Family: "}</span>{c.family.as_str()}</span><br/>
                    <span class="trait"><span>{"Trait: "}</span>{c.trait_name.as_str()}</span><br/>
                    <span class="material"><span>{"Material: "}</span>{c.material_name.as_str()}</span><br/>
                    <span class="sources"><span>{"Sources:"}</span>
                        <ul>
                            {c.sources.iter().map(|s| html! { <li>{s}</li> }).collect::<Html>()}
                        </ul>
                    </span>
                </div>
            </div>
            <span>
                {if props.icon {
                    html! { <ClassIcon value={c.class.clone()} /> }
                } else {
                    html! {}
                }}
                {c.creature.as_str()}
            </span>
        </div>
    }
}
