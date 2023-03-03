use implicit_clone::unsync::IString;
use qstring::QString;
use std::collections::BTreeSet;
use std::rc::Rc;

use yewdux::prelude::{init_listener, Listener, Reducer, Store};

use crate::embed_data;
use crate::save::Save;
use data::personality::Stat;
use data::r#trait::Trait;
use data::spell::Spell;
use data::Data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub primary_trait: Option<Trait>,
    pub fused_trait: Option<Trait>,
    pub artifact_trait: Option<Trait>,
    pub personality_positive: Option<Stat>,
    pub personality_negative: Option<Stat>,
    pub spells: Vec<Spell>,
}

fn flatten_stat(a: Option<u8>, b: Option<u8>) -> Option<u8> {
    match (a, b) {
        (Some(x), Some(y)) => Some(((x + y) as f64 / 2f64) as u8),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

impl Member {
    pub fn new() -> Member {
        Self {
            primary_trait: None,
            fused_trait: None,
            artifact_trait: None,
            personality_positive: None,
            personality_negative: None,
            spells: Vec::new(),
        }
    }

    pub fn get_creature(&self, i: usize) -> &Option<Trait> {
        match i {
            0 => &self.primary_trait,
            1 => &self.fused_trait,
            2 => &self.artifact_trait,
            _ => &None,
        }
    }
    pub fn set_creature(&mut self, i: usize, c: &Option<Trait>) {
        match i {
            0 => self.primary_trait = c.clone(),
            1 => self.fused_trait = c.clone(),
            2 => self.artifact_trait = c.clone(),
            _ => (),
        }
    }

    pub fn selection(&self) -> Vec<i32> {
        vec![
            self.primary_trait.clone(),
            self.fused_trait.clone(),
            self.artifact_trait.clone(),
        ]
        .iter()
        .flatten()
        .map(|t| t.id)
        .collect()
    }

    pub fn class(&self) -> Option<IString> {
        self.fused_trait.as_ref().map(|x| x.class.clone())
    }
    pub fn health(&self) -> Option<u8> {
        flatten_stat(
            self.primary_trait.as_ref().map(|a| a.health()).flatten(),
            self.fused_trait.as_ref().map(|a| a.health()).flatten(),
        )
    }
    pub fn attack(&self) -> Option<u8> {
        flatten_stat(
            self.primary_trait.as_ref().map(|a| a.attack()).flatten(),
            self.fused_trait.as_ref().map(|a| a.attack()).flatten(),
        )
    }
    pub fn intelligence(&self) -> Option<u8> {
        flatten_stat(
            self.primary_trait
                .as_ref()
                .map(|a| a.intelligence())
                .flatten(),
            self.fused_trait
                .as_ref()
                .map(|a| a.intelligence())
                .flatten(),
        )
    }
    pub fn defense(&self) -> Option<u8> {
        flatten_stat(
            self.primary_trait.as_ref().map(|a| a.defense()).flatten(),
            self.fused_trait.as_ref().map(|a| a.defense()).flatten(),
        )
    }
    pub fn speed(&self) -> Option<u8> {
        flatten_stat(
            self.primary_trait.as_ref().map(|a| a.speed()).flatten(),
            self.fused_trait.as_ref().map(|a| a.speed()).flatten(),
        )
    }
    pub fn personality_for(&self, stat: Stat) -> Option<bool> {
        if self.personality_positive == Some(stat.clone()) {
            Some(true)
        } else if self.personality_negative == Some(stat.clone()) {
            Some(false)
        } else {
            None
        }
    }

    pub fn set_spell(&mut self, i: usize, c: &Option<Spell>) {
        let spells = if let Some(s) = c {
            let mut spells = self.spells.to_vec();
            if i < spells.len() {
                spells[i] = s.clone();
            } else {
                spells.push(s.clone());
            }
            spells
        } else {
            let mut spells = self.spells.to_vec();
            if i < spells.len() {
                spells.remove(i);
            }
            spells
        };
        self.spells = spells;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub data: Rc<Data>,
    pub party: Vec<Member>,
    pub trait_pool: Vec<Option<Trait>>,
}

impl Store for State {
    fn new() -> Self {
        let data = Rc::new(embed_data::load());
        let storage = UrlStorageListener { data: data.clone() };
        init_listener(storage.clone());
        storage.load().unwrap_or(State::new(data.clone()))
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}

impl State {
    fn new(data: Rc<Data>) -> Self {
        Self {
            data,
            party: vec![
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
            ],
            trait_pool: vec![None], // TODO: no Option
        }
    }

    pub fn traits_selection(&self) -> BTreeSet<i32> {
        let a: Vec<i32> = self.party.iter().map(|m| m.selection()).flatten().collect();
        let b: Vec<i32> = self.trait_pool.iter().flatten().map(|t| t.id).collect();
        a.into_iter().chain(b).collect()
    }

    pub fn spells_selection(&self) -> BTreeSet<i16> {
        BTreeSet::new() // TODO
    }
}

#[derive(Clone)]
struct UrlStorageListener {
    data: Rc<Data>,
}

impl UrlStorageListener {
    pub fn load(&self) -> Option<State> {
        let location: web_sys::Location = web_sys::window().unwrap().location();
        let qs = QString::from(location.search().unwrap().as_str());
        if let Some(s) = qs.get("s") {
            let maybe_save = Save::from_string(&String::from(s));
            if let Ok(save) = maybe_save {
                Some(save.as_state(self.data.clone()))
            } else {
                log::warn!("failed to load save: {:?}", maybe_save);
                None
            }
        } else {
            None
        }
    }

    pub fn save(&self, state: &State) {
        let location: web_sys::Location = web_sys::window().unwrap().location();
        let history: web_sys::History = web_sys::window().unwrap().history().unwrap();
        let save_string = Save::from_state(&state, &self.data).as_string();
        history
            .replace_state_with_url(
                &wasm_bindgen::JsValue::null(),
                "",
                Some(format!("{}?s={}", location.pathname().unwrap(), save_string).as_str()),
            )
            .unwrap();
    }
}

impl Listener for UrlStorageListener {
    type Store = State;

    fn on_change(&mut self, state: Rc<Self::Store>) {
        self.save(&state);
    }
}

pub enum Action {
    Set((usize, usize, Trait)),
    Clear((usize, usize)),
    Swap((usize, usize, usize, usize)),
    SetPersonality((usize, Stat, bool)),
    SetSpell((usize, usize, Spell)),
    ClearSpell((usize, usize)),
}

impl Reducer<State> for Action {
    fn apply(self, mut state0: Rc<State>) -> Rc<State> {
        let state = Rc::make_mut(&mut state0);

        match self {
            Action::Set((position, index, r#trait)) => {
                if let Some(member) = state.party.get(position) {
                    let mut member = member.clone();
                    member.set_creature(index, &Some(r#trait));
                    state.party[position] = member;
                } else {
                    state.trait_pool[index] = Some(r#trait);
                    state.trait_pool = normalize_trait_pool(state.trait_pool.clone());
                }
            }
            Action::Clear((position, index)) => {
                if let Some(member) = state.party.get(position) {
                    let mut member = member.clone();
                    member.set_creature(index, &None);
                    state.party[position] = member;
                } else {
                    state.trait_pool[index] = None;
                    state.trait_pool = normalize_trait_pool(state.trait_pool.clone());
                }
            }
            Action::Swap((from_position, from_index, to_position, to_index)) => {
                if from_position == to_position {
                    if let Some(member) = state.party.get(from_position) {
                        let from = member.get_creature(from_index);
                        let to = member.get_creature(to_index);

                        let mut m = member.clone();
                        m.set_creature(from_index, &to);
                        m.set_creature(to_index, &from);

                        state.party[from_position] = m;
                    } else {
                        let from = state.trait_pool.get(from_index).unwrap().clone();
                        let to = state.trait_pool.get(to_index).unwrap().clone();

                        state.trait_pool[from_index] = to;
                        state.trait_pool[to_index] = from;
                        state.trait_pool = normalize_trait_pool(state.trait_pool.clone());
                    }
                } else {
                    match (
                        state.party.get(from_position).map(|x| x.clone()),
                        state.party.get(to_position).map(|x| x.clone()),
                    ) {
                        (Some(from_member), Some(to_member)) => {
                            let from = from_member.get_creature(from_index);
                            let to = to_member.get_creature(to_index);

                            let mut f = from_member.clone();
                            f.set_creature(from_index, &to);
                            let mut t = to_member.clone();
                            t.set_creature(to_index, &from);

                            state.party[to_position] = t;
                            state.party[from_position] = f;
                        }
                        (Some(from_member), None) => {
                            let from = from_member.get_creature(from_index);
                            let to = state.trait_pool.get(to_index).unwrap().clone();

                            let mut f = from_member.clone();
                            f.set_creature(from_index, &to);

                            state.party[from_position] = f;
                            state.trait_pool[to_index] = from.clone();
                            state.trait_pool = normalize_trait_pool(state.trait_pool.clone());
                        }
                        (None, Some(to_member)) => {
                            let to = to_member.get_creature(to_index);
                            let from = state.trait_pool.get(from_index).unwrap();

                            let mut t = to_member.clone();
                            t.set_creature(to_index, &from);

                            state.party[to_position] = t;
                            state.trait_pool[from_index] = to.clone();
                            state.trait_pool = normalize_trait_pool(state.trait_pool.clone());
                        }
                        (None, None) => (),
                    }
                }
            }
            Action::SetPersonality((index, stat, positive)) => {
                let mut m = state.party.get_mut(index).unwrap();
                if positive {
                    m.personality_positive = Some(stat);
                } else {
                    m.personality_negative = Some(stat);
                }
            }
            Action::SetSpell((position, index, spell)) => {
                if let Some(member) = state.party.get(position) {
                    let mut member = member.clone();
                    member.set_spell(index, &Some(spell));
                    state.party[position] = member;
                }
            }
            Action::ClearSpell((position, index)) => {
                if let Some(member) = state.party.get(position) {
                    let mut member = member.clone();
                    member.set_spell(index, &None);
                    state.party[position] = member;
                }
            }
        };

        state0
    }
}

fn normalize_trait_pool(trait_pool: Vec<Option<Trait>>) -> Vec<Option<Trait>> {
    let mut trait_pool = trait_pool
        .into_iter()
        .filter(|t| t.is_some())
        .collect::<Vec<Option<Trait>>>();
    trait_pool.push(None);
    trait_pool
}
