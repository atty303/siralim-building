mod data;
mod member;
mod party;
mod state;

use crate::data::Creature;
use crate::member::Member;
use crate::party::{Party, PartySwapEvent};
use crate::state::{Action, State};
use qstring::QString;
use std::ops::Deref;
use std::ptr::null;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let props = AppProps {
        creatures: data::Data::creatures(),
    };

    yew::Renderer::<App>::with_props(props).render();
}

#[derive(Properties, PartialEq)]
struct AppProps {
    creatures: Vec<Creature>,
}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
    let show_creatures_modal = use_state(|| false);
    let location: web_sys::Location = web_sys::window().unwrap().location();
    let history: web_sys::History = web_sys::window().unwrap().history().unwrap();
    log::debug!("{:?}", location.search());

    let qs = QString::from(location.search().unwrap().as_str());
    if let Some(s) = qs.get("s") {
        log::debug!("{:?}", s);
        //String::from(s)
    }

    let state = use_reducer(|| State::new(&props.creatures));

    use_effect_with_deps(
        move |state| {
            let save: String = String::from(state.deref().deref());
            history
                .replace_state_with_url(
                    &wasm_bindgen::JsValue::null(),
                    "",
                    Some(format!("/?s={}", save).as_str()),
                )
                .unwrap();
            || ()
        },
        state.clone(),
    );

    let on_swap = {
        let state = state.clone();
        let party = state.party.clone();
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

    let open_creatures_modal = move |_| {
        show_creatures_modal.set(true);
    };
    html! {
        <div>
            <button onclick={open_creatures_modal}>{"open"}</button>
            <Party
                party={state.party.clone()}
                on_swap={on_swap}
            />
        </div>
    }
}

//         CreatureModal {
//             items: &cx.props.creatures,
//             show: **show_creatures_modal,
//             on_select: move |_| {
//                 show_creatures_modal.set(false);
//             }
//         }
