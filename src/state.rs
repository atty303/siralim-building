use crate::data::Creature;
use crate::member::Member;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub party: Vec<Member>,
}

impl State {
    pub fn new(creatures: &Vec<Creature>) -> State {
        State {
            party: vec![
                Member {
                    primary_creature: Some(creatures.get(0).unwrap().clone()),
                    fused_creature: None,
                    artifact_creature: None,
                },
                Member {
                    primary_creature: None,
                    fused_creature: Some(creatures.get(100).unwrap().clone()),
                    artifact_creature: None,
                },
                Member {
                    primary_creature: None,
                    fused_creature: None,
                    artifact_creature: Some(creatures.get(200).unwrap().clone()),
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
