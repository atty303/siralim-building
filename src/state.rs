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

impl Member {
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub party: Vec<Member>,
}

impl State {
    pub fn new(data: &Data) -> State {
        State {
            party: vec![
                Member {
                    primary_trait: Some(
                        data.get_trait(17346231099549687105u64 as i64)
                            .unwrap()
                            .clone(),
                    ),
                    fused_trait: None,
                    artifact_trait: None,
                },
                Member {
                    primary_trait: None,
                    fused_trait: None,
                    artifact_trait: None,
                },
                Member {
                    primary_trait: None,
                    fused_trait: None,
                    artifact_trait: None,
                },
                Member {
                    primary_trait: None,
                    fused_trait: None,
                    artifact_trait: None,
                },
                Member {
                    primary_trait: None,
                    fused_trait: None,
                    artifact_trait: None,
                },
                Member {
                    primary_trait: None,
                    fused_trait: None,
                    artifact_trait: None,
                },
            ],
        }
    }
}

pub enum Action {
    Swap((usize, usize, usize, usize)),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Swap((from_position, from_index, to_position, to_index)) => {
                if from_position == to_position {
                    let member = self.party.get(from_position).unwrap().clone();
                    let from = member.get_creature(from_index);
                    let to = member.get_creature(to_index);

                    let mut m = member.clone();
                    m.set_creature(from_index, &to);
                    m.set_creature(to_index, &from);

                    let mut p = self.party.to_vec();
                    p[from_position] = m;

                    State { party: p }.into()
                } else {
                    let from_member = self.party.get(from_position).unwrap().clone();
                    let from = from_member.get_creature(from_index);
                    let to_member = self.party.get(to_position).unwrap().clone();
                    let to = to_member.get_creature(to_index);

                    let mut f = from_member.clone();
                    f.set_creature(from_index, &to);
                    let mut t = to_member.clone();
                    t.set_creature(to_index, &from);

                    let mut p = self.party.to_vec();
                    p[to_position] = t;
                    p[from_position] = f;

                    State { party: p }.into()
                }
            }
        }
    }
}
