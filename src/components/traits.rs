use std::collections::BTreeSet;
use std::rc::Rc;

use implicit_clone::unsync::IString;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use data::r#trait::Trait;
use data::Data;

use crate::components::creature_name::CreatureName;
use crate::components::description::Description;
use crate::components::icon::ClassIcon;
use crate::components::modal::Modal;

#[derive(Clone)]
pub struct TraitSelectEvent {
    pub r#trait: Trait,
}

#[derive(Properties, PartialEq)]
pub struct TraitsModalProps {
    pub show: bool,
    pub selection: BTreeSet<i32>,
    pub on_cancel: Callback<()>,
    pub on_select: Callback<TraitSelectEvent>,
}

#[function_component(TraitsModal)]
pub fn traits_modal(props: &TraitsModalProps) -> Html {
    let data = use_context::<Rc<Data>>().unwrap();
    let query = use_state(|| IString::from("class:death"));
    let items = use_state(|| Vec::<Trait>::new());

    {
        let items = items.clone();
        use_effect_with_deps(
            move |query| {
                log::debug!("search: {}", query.as_str());
                if let Ok(xs) = data.search_trait(query.as_str()) {
                    items.set(xs);
                }
                || ()
            },
            query.clone(),
        );
    }

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };
    let on_select = {
        let on_select = props.on_select.clone();
        Callback::from(move |e| on_select.emit(e))
    };
    let onkeypress = {
        let query = query.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                query.set(IString::from(value));
            }
        })
    };

    html! {
        <Modal
            show={props.show}
            class={classes!("trait-spell-modal",  "trait")}
            on_request_close={on_cancel}
        >
            <div class="top">
                <input type="text" onkeypress={onkeypress}/>
                <span>{items.len()} {" items"}</span>
            </div>
            <div class="table">
                <TraitTable
                    items={(*items).clone()}
                    selection={props.selection.clone()}
                    on_select={on_select}
                />
            </div>
        </Modal>
    }
}

#[derive(Properties, PartialEq)]
struct TraitTableProps {
    items: Vec<Trait>,
    selection: BTreeSet<i32>,
    on_select: Callback<TraitSelectEvent>,
}

#[function_component(TraitTable)]
fn trait_table(props: &TraitTableProps) -> Html {
    let on_click = |t: Trait| {
        let on_select = props.on_select.clone();
        let e = TraitSelectEvent { r#trait: t };
        Callback::from(move |_| on_select.emit(e.clone()))
    };

    html! {
        <table class="trait">
            <thead>
                <tr>
                    <th></th>
                    <th>{"Class"}</th>
                    <th>{"Family"}</th>
                    <th>{"Creature"}</th>
                    <th>{"Trait Description"}</th>
                    <th><img src="image/health.png" /></th>
                    <th><img src="image/attack.png" /></th>
                    <th><img src="image/intelligence.png" /></th>
                    <th><img src="image/defense.png" /></th>
                    <th><img src="image/speed.png" /></th>
                </tr>
            </thead>
            <tbody>
                {props.items.iter().map(|c| {
                    let selected = props.selection.contains(&c.id);
                    let classes = if selected { classes!("selected") } else { Classes::new() };
                    html! {
                        <tr class={classes}>
                            <td class="select" onclick={on_click(c.clone()).clone()}><Icon icon_id={IconId::BootstrapCheckSquareFill} /></td>
                            <td class="class">
                                <ClassIcon value={c.class.clone()} />
                                {c.class.clone()}
                            </td>
                            <td class="family">{c.family.clone()}</td>
                            <td class="creature"><CreatureName r#trait={c.clone()} icon={false} /></td>
                            <td class="trait_description">
                                <Description value={c.trait_description.clone()} />
                            </td>
                            <td class="stat health"><CreatureStat value={c.stats.as_ref().map(|s| s.health)} /></td>
                            <td class="stat attack"><CreatureStat value={c.stats.as_ref().map(|s| s.attack)} /></td>
                            <td class="stat intelligence"><CreatureStat value={c.stats.as_ref().map(|s| s.intelligence)} /></td>
                            <td class="stat defense"><CreatureStat value={c.stats.as_ref().map(|s| s.defense)} /></td>
                            <td class="stat speed"><CreatureStat value={c.stats.as_ref().map(|s| s.speed)} /></td>
                        </tr>
                    }
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
