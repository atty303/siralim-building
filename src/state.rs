use implicit_clone::unsync::IString;
use std::rc::Rc;

use yew::prelude::*;

use data::r#trait::Trait;
use data::Data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub primary_trait: Option<Trait>,
    pub fused_trait: Option<Trait>,
    pub artifact_trait: Option<Trait>,
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub party: Vec<Member>,
    pub trait_pool: Vec<Option<Trait>>,
}

impl State {
    pub fn new(_data: &Data) -> State {
        State {
            party: vec![
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
            ],
            trait_pool: vec![None],
        }
    }
}

pub enum Action {
    Set((usize, usize, Trait)),
    Clear((usize, usize)),
    Swap((usize, usize, usize, usize)),
}

fn normalize_trait_pool(trait_pool: Vec<Option<Trait>>) -> Vec<Option<Trait>> {
    let mut trait_pool = trait_pool
        .into_iter()
        .filter(|t| t.is_some())
        .collect::<Vec<Option<Trait>>>();
    trait_pool.push(None);
    trait_pool
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Set((position, index, r#trait)) => {
                if let Some(member) = self.party.get(position) {
                    let mut member = member.clone();
                    member.set_creature(index, &Some(r#trait));
                    let mut party = self.party.to_vec();
                    party[position] = member;
                    State {
                        party,
                        trait_pool: self.trait_pool.clone(),
                    }
                    .into()
                } else {
                    let mut trait_pool = self.trait_pool.clone();
                    trait_pool[index] = Some(r#trait);
                    State {
                        party: self.party.clone(),
                        trait_pool: normalize_trait_pool(trait_pool),
                    }
                    .into()
                }
            }
            Action::Clear((position, index)) => {
                if let Some(member) = self.party.get(position) {
                    let mut member = member.clone();
                    member.set_creature(index, &None);
                    let mut party = self.party.to_vec();
                    party[position] = member;
                    State {
                        party,
                        trait_pool: self.trait_pool.clone(),
                    }
                    .into()
                } else {
                    let mut trait_pool = self.trait_pool.clone();
                    trait_pool[index] = None;
                    State {
                        party: self.party.clone(),
                        trait_pool: normalize_trait_pool(trait_pool),
                    }
                    .into()
                }
            }
            Action::Swap((from_position, from_index, to_position, to_index)) => {
                if from_position == to_position {
                    if let Some(member) = self.party.get(from_position) {
                        let from = member.get_creature(from_index);
                        let to = member.get_creature(to_index);

                        let mut m = member.clone();
                        m.set_creature(from_index, &to);
                        m.set_creature(to_index, &from);

                        let mut party = self.party.to_vec();
                        party[from_position] = m;

                        State {
                            party,
                            trait_pool: self.trait_pool.clone(),
                        }
                        .into()
                    } else {
                        let from = self.trait_pool.get(from_index).unwrap();
                        let to = self.trait_pool.get(to_index).unwrap();

                        let mut trait_pool = self.trait_pool.to_vec();
                        trait_pool[from_index] = to.clone();
                        trait_pool[to_index] = from.clone();

                        State {
                            party: self.party.clone(),
                            trait_pool: normalize_trait_pool(trait_pool),
                        }
                        .into()
                    }
                } else {
                    match (self.party.get(from_position), self.party.get(to_position)) {
                        (Some(from_member), Some(to_member)) => {
                            let from = from_member.get_creature(from_index);
                            let to = to_member.get_creature(to_index);

                            let mut f = from_member.clone();
                            f.set_creature(from_index, &to);
                            let mut t = to_member.clone();
                            t.set_creature(to_index, &from);

                            let mut p = self.party.to_vec();
                            p[to_position] = t;
                            p[from_position] = f;

                            State {
                                party: p,
                                trait_pool: self.trait_pool.clone(),
                            }
                            .into()
                        }
                        (Some(from_member), None) => {
                            let from = from_member.get_creature(from_index);
                            let to = self.trait_pool.get(to_index).unwrap();

                            let mut f = from_member.clone();
                            f.set_creature(from_index, &to);

                            let mut party = self.party.to_vec();
                            party[from_position] = f;

                            let mut trait_pool = self.trait_pool.clone();
                            trait_pool[to_index] = from.clone();

                            State {
                                party,
                                trait_pool: normalize_trait_pool(trait_pool),
                            }
                            .into()
                        }
                        (None, Some(to_member)) => {
                            let to = to_member.get_creature(to_index);
                            let from = self.trait_pool.get(from_index).unwrap();

                            let mut t = to_member.clone();
                            t.set_creature(to_index, &from);

                            let mut party = self.party.to_vec();
                            party[to_position] = t;

                            let mut trait_pool = self.trait_pool.clone();
                            trait_pool[from_index] = to.clone();

                            State {
                                party,
                                trait_pool: normalize_trait_pool(trait_pool),
                            }
                            .into()
                        }
                        (None, None) => self,
                    }
                }
            }
        }
    }
}
