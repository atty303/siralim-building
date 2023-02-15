use std::error::Error;
use std::io::{Read, Write};
use std::rc::Rc;

use apache_avro::{AvroSchema, Schema};
use base64::Engine;
use serde::{Deserialize, Serialize, Serializer};
use yew::prelude::*;

use crate::data::{Creature, Data};
use crate::member::Member;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub party: Vec<Member>,
}

impl State {
    pub fn new(data: &Data) -> State {
        State {
            party: vec![
                Member {
                    primary_creature: Some(data.creatures.get(0).unwrap().clone()),
                    fused_creature: None,
                    artifact_creature: None,
                },
                Member {
                    primary_creature: None,
                    fused_creature: Some(data.creatures.get(100).unwrap().clone()),
                    artifact_creature: None,
                },
                Member {
                    primary_creature: None,
                    fused_creature: None,
                    artifact_creature: Some(data.creatures.get(200).unwrap().clone()),
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
    primary_creature: Option<String>,
    fused_creature: Option<String>,
    artifact_creature: Option<String>,
}

impl State {
    pub fn from(save: &Save, data: &Data) -> State {
        State {
            party: save
                .party
                .iter()
                .map(|m| Member {
                    primary_creature: m
                        .primary_creature
                        .as_ref()
                        .map(|uid| data.get_creature_by_uid(&uid))
                        .flatten()
                        .map(|c| c.clone()),
                    fused_creature: m
                        .fused_creature
                        .as_ref()
                        .map(|uid| data.get_creature_by_uid(&uid))
                        .flatten()
                        .map(|c| c.clone()),
                    artifact_creature: m
                        .artifact_creature
                        .as_ref()
                        .map(|uid| data.get_creature_by_uid(&uid))
                        .flatten()
                        .map(|c| c.clone()),
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
                    primary_creature: m.primary_creature.clone().map(|c| c.uid),
                    fused_creature: m.fused_creature.clone().map(|c| c.uid),
                    artifact_creature: m.artifact_creature.clone().map(|c| c.uid),
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
