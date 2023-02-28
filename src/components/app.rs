use std::rc::Rc;

use data::personality::Stat;
use qstring::QString;
use yew::prelude::*;

use crate::components::party::{Party, PartySwapEvent, PartyTraitEvent};
use crate::components::traits::{TraitSelectEvent, TraitsModal};
use crate::save::Save;
use crate::state::{Action, State};

#[derive(Properties, PartialEq)]
pub struct AppProps {
    pub data: data::Data,
}

#[function_component(App)]
pub fn app(props: &AppProps) -> Html {
    let show_modal = use_state(|| false);
    use_effect_with_deps(
        move |v| {
            let class = if **v { "open-modal" } else { "" };
            let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
            let body = document.body().unwrap();
            body.set_class_name(class);
        },
        show_modal.clone(),
    );

    let show_traits_modal = use_state(|| true);

    let location: web_sys::Location = web_sys::window().unwrap().location();
    let history: web_sys::History = web_sys::window().unwrap().history().unwrap();

    let qs = QString::from(location.search().unwrap().as_str());
    let loaded_state = if let Some(s) = qs.get("s") {
        let maybe_save = Save::from_string(&String::from(s));
        if let Ok(save) = maybe_save {
            Some(save.as_state(&props.data))
        } else {
            log::warn!("failed to load save: {:?}", maybe_save);
            None
        }
    } else {
        None
    };
    let initial_state = loaded_state.unwrap_or_else(|| State::new(&props.data));

    let data = use_memo(|_| props.data.clone(), ());

    let state = use_reducer(|| initial_state);
    let d = data.clone();
    use_effect_with_deps(
        move |state| {
            let save_string = Save::from_state(state, d.as_ref()).as_string();
            history
                .replace_state_with_url(
                    &wasm_bindgen::JsValue::null(),
                    "",
                    Some(format!("{}?s={}", location.pathname().unwrap(), save_string).as_str()),
                )
                .unwrap();
            || ()
        },
        state.clone(),
    );

    let clicked_member = use_state(|| None);
    let on_member_click = {
        let clicked_member = clicked_member.clone();
        let show_modal = show_modal.clone();
        let show_traits_modal = show_traits_modal.clone();
        Callback::from(move |e: PartyTraitEvent| {
            clicked_member.set(Some(e));
            show_modal.set(true);
            show_traits_modal.set(true);
        })
    };
    let on_member_clear = {
        let state = state.clone();
        Callback::from(move |e: PartyTraitEvent| {
            state.dispatch(Action::Clear((e.position, e.index)));
        })
    };

    let on_swap = {
        let state = state.clone();
        Callback::from(move |e: PartySwapEvent| {
            log::debug!("on_swap: {:?}", e);
            state.dispatch(Action::Swap((
                e.from_position,
                e.from_index,
                e.to_position,
                e.to_index,
            )));
        })
    };

    let on_close_traits_modal = {
        let show_modal = show_modal.clone();
        let show_traits_modal = show_traits_modal.clone();
        Callback::from(move |_| {
            show_modal.set(false);
            show_traits_modal.set(false);
        })
    };
    let on_select_trait = {
        let show_modal = show_modal.clone();
        let show_traits_modal = show_traits_modal.clone();
        let clicked_member = clicked_member.clone();
        let state = state.clone();
        Callback::from(move |e: TraitSelectEvent| {
            show_modal.set(false);
            show_traits_modal.set(false);
            if let Some(t) = clicked_member.as_ref() {
                state.dispatch(Action::Set((t.position, t.index, e.r#trait)));
            }
        })
    };

    let dispatch_set_personality = {
        let state = state.clone();
        Callback::from(move |x: (usize, Stat, bool)| state.dispatch(Action::SetPersonality(x)))
    };

    html! {
        <ContextProvider<Rc<data::Data>> context={data}>
            <Party
                party={state.party.clone()}
                pool={state.trait_pool.clone()}
                on_swap={on_swap}
                on_click={on_member_click}
                on_clear={on_member_clear}
                {dispatch_set_personality}
            />
            <TraitsModal
                show={*show_traits_modal}
                on_cancel={on_close_traits_modal}
                on_select={on_select_trait}
            />
        </ContextProvider<Rc<data::Data>>>
    }
}
