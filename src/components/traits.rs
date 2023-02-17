use yew::prelude::*;

use data::r#trait::Trait;
use data::Data;

use crate::components::icon::ClassIcon;
use crate::components::modal::Modal;

#[derive(Clone)]
pub struct TraitSelectEvent {
    pub r#trait: Trait,
}

#[derive(Properties, PartialEq)]
pub struct TraitsModalProps {
    pub data: Data,
    pub show: bool,
    pub on_cancel: Callback<()>,
    pub on_select: Callback<TraitSelectEvent>,
}

#[function_component(TraitsModal)]
pub fn traits_modal(props: &TraitsModalProps) -> Html {
    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };
    let on_select = {
        let on_select = props.on_select.clone();
        Callback::from(move |e| on_select.emit(e))
    };

    html! {
        <Modal
            show={props.show}
            on_request_close={on_cancel}
        >
            <input/>
            <TraitTable
                data={props.data.clone()}
                on_select={on_select}
            />
        </Modal>
    }
}

#[derive(Properties, PartialEq)]
pub struct TraitTableProps {
    data: Data,
    on_select: Callback<TraitSelectEvent>,
}

#[function_component(TraitTable)]
pub fn trait_table(props: &TraitTableProps) -> Html {
    let items = use_state(|| Vec::<Trait>::new());

    let on_click = |t: Trait| {
        let on_select = props.on_select.clone();
        let e = TraitSelectEvent { r#trait: t };
        Callback::from(move |_| on_select.emit(e.clone()))
    };

    html! {
        <table class="trait">
            <thead>
                <tr>
                    <th>{"Class"}</th>
                    <th>{"Family"}</th>
                    <th>{"Creature"}</th>
                    <th>{"Trait"}</th>
                    <th>{"Trait Description"}</th>
                    <th><img src="image/health.png" /></th>
                    <th><img src="image/attack.png" /></th>
                    <th><img src="image/intelligence.png" /></th>
                    <th><img src="image/defense.png" /></th>
                    <th><img src="image/speed.png" /></th>
                    <th>{"Material Name"}</th>
                </tr>
            </thead>
            <tbody>
                {items.iter().map(|c| html! {
                    <tr onclick={on_click(c.clone()).clone()}>
                        <td class="class">
                            <ClassIcon value={c.class.clone()} />
                            {c.class.clone()}
                        </td>
                        <td class="family">{c.family.clone()}</td>
                        <td class="creature">{c.creature.clone()}</td>
                        <td class="trait">{c.trait_name.clone()}</td>
                        <td class="trait_description">{c.trait_description.clone()}</td>
                        <td class="stat health"><CreatureStat value={c.stats.as_ref().map(|s| s.health)} /></td>
                        <td class="stat attack"><CreatureStat value={c.stats.as_ref().map(|s| s.attack)} /></td>
                        <td class="stat intelligence"><CreatureStat value={c.stats.as_ref().map(|s| s.intelligence)} /></td>
                        <td class="stat defense"><CreatureStat value={c.stats.as_ref().map(|s| s.defense)} /></td>
                        <td class="stat speed"><CreatureStat value={c.stats.as_ref().map(|s| s.speed)} /></td>
                        <td class="material_name">{c.material_name.clone()}</td>
                    </tr>
                }).collect::<Html>()}
            </tbody>
        </table>
    }
}

#[derive(Properties, PartialEq)]
struct CreatureStatProps {
    value: Option<u8>,
}

#[function_component(CreatureStat)]
fn creature_stat(props: &CreatureStatProps) -> Html {
    if let Some(value) = props.value {
        html! { value }
    } else {
        html! { {"-"} }
    }
}
