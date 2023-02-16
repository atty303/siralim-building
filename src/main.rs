mod embed_data;
mod embed_directory;
mod member;
mod party;
mod state;

use crate::member::Member;
use crate::party::{Party, PartySwapEvent};
use crate::state::{Action, Save, State};
use data::Creature;
use implicit_clone::unsync::IString;
use qstring::QString;
use std::ops::Deref;
use std::ptr::null;
use tantivy::query::QueryParser;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let props = AppProps {
        data: embed_data::EmbedData::load(),
    };

    yew::Renderer::<App>::with_props(props).render();
}

#[derive(Properties, PartialEq)]
struct AppProps {
    data: data::Data,
}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
    let show_creatures_modal = use_state(|| false);
    let location: web_sys::Location = web_sys::window().unwrap().location();
    let history: web_sys::History = web_sys::window().unwrap().history().unwrap();
    log::debug!("{:?}", location.search());

    let dir = embed_directory::EmbedDirectory::new(embed_data::EmbedTraits);
    let index = tantivy::Index::open(dir).unwrap();
    let reader = index.reader().unwrap();
    let searcher = reader.searcher();
    for r in searcher.segment_readers().iter() {
        //r.doc_ids_alive().
    }
    let a: AttrValue = IString::default();

    let query_parser = QueryParser::for_index(
        &index,
        vec![index.schema().get_field("trait_description").unwrap()],
    );
    let query = query_parser.parse_query("spell").unwrap();

    let docs = searcher
        .search(&query, &tantivy::collector::TopDocs::with_limit(10))
        .unwrap();

    for (score, doc_address) in docs {
        let doc = searcher.doc(doc_address).unwrap();
        log::debug!("{}: {}", score, index.schema().to_json(&doc));
    }

    let qs = QString::from(location.search().unwrap().as_str());
    let loaded_state = if let Some(s) = qs.get("s") {
        log::debug!("save: {:?}", s);
        let maybe_save = Save::from(&String::from(s));
        if let Ok(save) = maybe_save {
            Some(State::from(&save, &props.data))
        } else {
            log::warn!("failed to load save: {:?}", maybe_save);
            None
        }
    } else {
        None
    };
    let initial_state = loaded_state.unwrap_or_else(|| State::new(&props.data));

    let state = use_reducer(|| initial_state);

    use_effect_with_deps(
        move |state| {
            let save_string = state.as_save().as_string();
            history
                .replace_state_with_url(
                    &wasm_bindgen::JsValue::null(),
                    "",
                    Some(format!("/?s={}", save_string).as_str()),
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
