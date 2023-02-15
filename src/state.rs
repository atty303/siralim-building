use std::io::Write;
use std::rc::Rc;

use apache_avro::Schema::Record;
use apache_avro::{AvroSchema, Schema};
use base64::Engine;
use serde::{Deserialize, Serialize, Serializer};
use yew::prelude::*;

use crate::data::Creature;
use crate::member::Member;

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

// static RAW_SCHEMA: &'static str = r#"
// {
//     "type": "record",
//     "name": "Save",
//     "fields": [{
//         "name": "party",
//         "type": "record",
//         "fields": [
//             {
//                 "name": "members",
//                 "type": "array",
//                 "items": {
//                     "type": "record",
//                     "name": "SaveMember",
//                     "fields": [
//                        {"name": "primary_creature", "type": ["string", "null"]},
//                        {"name": "fused_creature", "type": ["string", "null"]},
//                        {"name": "artifact_creature", "type": ["string", "null"]}
//                     ]
//                 }
//             }
//         ]
//     }]
// }
// "#;
//
// static SCHEMA: Lazy<Schema> = Lazy::new(|| Schema::parse_str(RAW_SCHEMA).unwrap());
//
#[derive(Serialize, Deserialize, AvroSchema)]
struct Save {
    party: SaveParty,
}

#[derive(Serialize, Deserialize, AvroSchema)]
struct SaveParty {
    members: Vec<SaveMember>,
}

#[derive(Serialize, Deserialize, AvroSchema)]
struct SaveMember {
    primary_creature: Option<String>,
    fused_creature: Option<String>,
    artifact_creature: Option<String>,
}

impl From<&State> for String {
    fn from(value: &State) -> Self {
        let save = Save {
            party: SaveParty {
                members: value
                    .party
                    .iter()
                    .map(|m| SaveMember {
                        primary_creature: m.primary_creature.clone().map(|c| c.uid),
                        fused_creature: m.fused_creature.clone().map(|c| c.uid),
                        artifact_creature: m.artifact_creature.clone().map(|c| c.uid),
                    })
                    .collect(),
            },
        };
        let save_value = apache_avro::to_value(save).unwrap();
        let bytes = apache_avro::to_avro_datum(&Save::get_schema(), save_value).unwrap();
        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        e.write_all(&bytes).unwrap();
        let c = e.finish().unwrap();
        base64::engine::general_purpose::URL_SAFE.encode(c)
    }
}

impl From<&String> for State {
    fn from(value: &String) -> Self {
        todo!()
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
