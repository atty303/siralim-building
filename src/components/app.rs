use std::rc::Rc;

use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::party::{Party, PartySpellEvent, PartySwapEvent, PartyTraitEvent};
use crate::components::spells::{SpellSelectEvent, SpellsModal};
use crate::components::traits::{TraitSelectEvent, TraitsModal};
use crate::state::{Action, State};

#[derive(Properties, PartialEq)]
pub struct AppProps {}

#[function_component(App)]
pub fn app(_props: &AppProps) -> Html {
    let (state, dispatch) = use_store::<State>();

    let show_traits_modal = use_state(|| false);
    let show_spells_modal = use_state(|| false);

    let data = &state.data;

    let clicked_member = use_state(|| None);
    let on_member_click = {
        let clicked_member = clicked_member.clone();
        let show_traits_modal = show_traits_modal.clone();
        Callback::from(move |e: PartyTraitEvent| {
            clicked_member.set(Some(e));
            show_traits_modal.set(true);
        })
    };
    let on_member_clear =
        dispatch.apply_callback(|e: PartyTraitEvent| Action::Clear((e.position, e.index)));

    let on_swap = dispatch.apply_callback(|e: PartySwapEvent| {
        Action::Swap((e.from_position, e.from_index, e.to_position, e.to_index))
    });

    let on_close_traits_modal = {
        let show_traits_modal = show_traits_modal.clone();
        Callback::from(move |_| {
            show_traits_modal.set(false);
        })
    };
    let on_select_trait = {
        let show_traits_modal = show_traits_modal.clone();
        let clicked_member = clicked_member.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |e: TraitSelectEvent| {
            show_traits_modal.set(false);
            if let Some(t) = clicked_member.as_ref() {
                dispatch.apply(Action::Set((t.position, t.index, e.r#trait)));
            }
        })
    };

    let clicked_spell_member = use_state(|| None);
    let on_spell_click = {
        let clicked_spell_member = clicked_spell_member.clone();
        let show_spells_modal = show_spells_modal.clone();
        Callback::from(move |e: PartySpellEvent| {
            clicked_spell_member.set(Some(e));
            show_spells_modal.set(true);
        })
    };
    let on_close_spells_modal = {
        let show_spells_modal = show_spells_modal.clone();
        Callback::from(move |_| {
            show_spells_modal.set(false);
        })
    };
    let on_select_spell = {
        let show_spells_modal = show_spells_modal.clone();
        let clicked_spell_member = clicked_spell_member.clone();
        let dispatch = dispatch.clone();
        Callback::from(move |e: SpellSelectEvent| {
            show_spells_modal.set(false);
            if let Some(t) = clicked_spell_member.as_ref() {
                dispatch.apply(Action::SetSpell((t.position, t.index, e.spell)));
            }
        })
    };

    html! {
        <ContextProvider<Rc<data::Data>> context={data}>
            <Party
                party={state.party.clone()}
                pool={state.trait_pool.clone()}
                on_swap={on_swap}
                on_click={on_member_click}
                on_clear={on_member_clear}
                {on_spell_click}
            />
            <TraitsModal
                show={*show_traits_modal}
                selection={state.traits_selection()}
                on_cancel={on_close_traits_modal}
                on_select={on_select_trait}
            />
            <SpellsModal
                show={*show_spells_modal}
                selection={state.spells_selection()}
                on_cancel={on_close_spells_modal}
                on_select={on_select_spell}
            />
        </ContextProvider<Rc<data::Data>>>
    }
}
