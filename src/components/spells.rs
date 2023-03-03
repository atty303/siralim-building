use std::collections::BTreeSet;
use std::rc::Rc;

use implicit_clone::unsync::IString;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use data::spell::Spell;
use data::Data;

use crate::components::description::Description;
use crate::components::icon::ClassIcon;
use crate::components::modal::Modal;
use crate::components::spell_name::SpellName;

#[derive(Clone)]
pub struct SpellSelectEvent {
    pub spell: Spell,
}

#[derive(Properties, PartialEq)]
pub struct SpellsModalProps {
    pub show: bool,
    pub selection: BTreeSet<i16>,
    pub on_cancel: Callback<()>,
    pub on_select: Callback<SpellSelectEvent>,
}

#[function_component(SpellsModal)]
pub fn spells_modal(props: &SpellsModalProps) -> Html {
    let data = use_context::<Rc<Data>>().unwrap();
    let query = use_state(|| IString::from("*"));
    let items = use_state(|| Vec::<Spell>::new());
    {
        let items = items.clone();
        use_effect_with_deps(
            move |query| {
                if let Ok(xs) = data.search_spell(query.as_str()) {
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
            class={classes!("trait-spell-modal", "spell")}
            on_request_close={on_cancel}
        >
            <div class="top">
                <input type="text" onkeypress={onkeypress}/>
                <span>{items.len()} {" items"}</span>
            </div>
            <div class="table">
                <SpellTable
                    items={(*items).clone()}
                    selection={props.selection.clone()}
                    on_select={on_select}
                />
            </div>
        </Modal>
    }
}

#[derive(Properties, PartialEq)]
struct SpellTableProps {
    items: Vec<Spell>,
    selection: BTreeSet<i16>,
    on_select: Callback<SpellSelectEvent>,
}

#[function_component(SpellTable)]
fn spell_table(props: &SpellTableProps) -> Html {
    let on_click = |s: Spell| {
        let on_select = props.on_select.clone();
        let e = SpellSelectEvent { spell: s };
        Callback::from(move |_| on_select.emit(e.clone()))
    };

    html! {
        <table class="spell">
            <thead>
                <tr>
                    <th></th>
                    <th>{"Class"}</th>
                    <th>{"Name"}</th>
                    <th>{"Description"}</th>
                    <th>{"Charges"}</th>
                </tr>
             </thead>
             <tbody>
                 {props.items.iter().map(|s| {
                     let selected = props.selection.contains(&s.id);
                     let classes = if selected { classes!("selected") } else { Classes::new() };
                     html! {
                         <tr class={classes}>
                             <td class="select" onclick={on_click(s.clone()).clone()}><Icon icon_id={IconId::BootstrapCheckSquareFill} /></td>
                             <td class="class">
                                 <ClassIcon value={s.class.clone()} />
                                 {s.class.clone()}
                             </td>
                             <td class="name"><SpellName spell={s.clone()} icon={false} /></td>
                             <td class="description">
                                 <Description value={s.description.clone()} />
                             </td>
                             <td class="charges">{format!("{}", s.charges)}</td>
                         </tr>
                     }
                 }).collect::<Html>()}
             </tbody>
        </table>
    }
}
