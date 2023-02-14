mod data;
mod member;
mod party;
mod state;

use crate::data::Creature;
use crate::member::Member;
use crate::party::{Party, PartySwapEvent};
use crate::state::{Action, State};
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

    let state = use_reducer(|| State::new(&props.creatures));

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

// fn App(cx: Scope<AppProps>) -> Element {
//     cx.render(rsx! {
//         div {
//             Party {
//                 party: party
//                 on_swap: move |e: PartyMemberSwapEvent| {
//                     log::info!("{:?}", e);
//                 }
//             }
//         }
//         CreatureModal {
//             items: &cx.props.creatures,
//             show: **show_creatures_modal,
//             on_select: move |_| {
//                 show_creatures_modal.set(false);
//             }
//         }
//     })
// }
