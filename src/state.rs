use std::error::Error;
use std::io::{Read, Write};
use std::rc::Rc;

use apache_avro::{AvroSchema, Schema};
use base64::Engine;
use serde::{Deserialize, Serialize, Serializer};
use yew::prelude::*;

use crate::member::Member;
use data::r#trait::Trait;
use data::{Creature, Data};

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
            ],
        }
    }
}

#[derive(Serialize, Deserialize, AvroSchema, Debug)]
pub struct Save {
    party: Vec<SaveMember>,
}

#[derive(Serialize, Deserialize, AvroSchema, Debug)]
struct SaveMember {
    primary_trait: Option<i64>,
    fused_trait: Option<i64>,
    artifact_trait: Option<i64>,
}

fn id_to_trait(data: &Data, id: Option<i64>) -> Option<Trait> {
    id.iter().flat_map(|i| data.get_trait(*i).ok()).next()
}

impl State {
    pub fn from(save: &Save, data: &Data) -> State {
        State {
            party: save
                .party
                .iter()
                .map(|m| Member {
                    primary_trait: id_to_trait(data, m.primary_trait),
                    fused_trait: id_to_trait(data, m.fused_trait),
                    artifact_trait: id_to_trait(data, m.artifact_trait),
                })
                .collect(),
        }
    }

    pub fn as_save(&self) -> Save {
        Save {
            party: self
                .party
                .iter()
                .map(|m| SaveMember {
                    primary_trait: m.primary_trait.clone().map(|c| c.id),
                    fused_trait: m.fused_trait.clone().map(|c| c.id),
                    artifact_trait: m.artifact_trait.clone().map(|c| c.id),
                })
                .collect(),
        }
    }
}

impl Save {
    pub fn from(value: &String) -> Result<Save, anyhow::Error> {
        let z_bytes = base64::engine::general_purpose::URL_SAFE.decode(value)?;
        let mut decoder = flate2::read::ZlibDecoder::new(std::io::Cursor::new(z_bytes));
        let mut bytes: Vec<u8> = Vec::new();
        decoder.read_to_end(&mut bytes)?;
        let avro_value = apache_avro::from_avro_datum(
            &Save::get_schema(),
            &mut std::io::Cursor::new(bytes),
            None,
        )?;
        let save: Save = apache_avro::from_value(&avro_value)?;
        Ok(save)
    }

    pub fn as_string(&self) -> String {
        let save_value = apache_avro::to_value(self).unwrap();
        let bytes = apache_avro::to_avro_datum(&Save::get_schema(), save_value).unwrap();
        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        e.write_all(&bytes).unwrap();
        let c = e.finish().unwrap();
        base64::engine::general_purpose::URL_SAFE.encode(c)
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
