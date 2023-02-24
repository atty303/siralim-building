use apache_avro::AvroSchema;
use base64::Engine;
use serde::{Deserialize, Serialize};

use data::r#trait::Trait;
use data::Data;

use crate::state::{Member, State};

#[derive(Serialize, Deserialize, AvroSchema, Debug)]
pub struct Save {
    party: Vec<SaveMember>,
    trait_pool: Vec<Option<i32>>,
}

#[derive(Serialize, Deserialize, AvroSchema, Debug)]
struct SaveMember {
    primary_trait: Option<i32>,
    fused_trait: Option<i32>,
    artifact_trait: Option<i32>,
}

fn id_to_trait(data: &Data, id: Option<i32>) -> Option<Trait> {
    id.iter().flat_map(|i| data.get_trait(*i).ok()).next()
}

impl Save {
    pub fn from_state(state: &State) -> Save {
        Save {
            party: state
                .party
                .iter()
                .map(|m| SaveMember {
                    primary_trait: m.primary_trait.clone().map(|c| c.id),
                    fused_trait: m.fused_trait.clone().map(|c| c.id),
                    artifact_trait: m.artifact_trait.clone().map(|c| c.id),
                })
                .collect(),
            trait_pool: state
                .trait_pool
                .iter()
                .map(|t| t.clone().map(|c| c.id))
                .collect(),
        }
    }

    pub fn as_state(&self, data: &Data) -> State {
        State {
            party: self
                .party
                .iter()
                .map(|m| Member {
                    primary_trait: id_to_trait(data, m.primary_trait),
                    fused_trait: id_to_trait(data, m.fused_trait),
                    artifact_trait: id_to_trait(data, m.artifact_trait),
                })
                .collect(),
            trait_pool: self
                .trait_pool
                .iter()
                .map(|id| id_to_trait(data, id.clone()))
                .collect(),
        }
    }

    pub fn from_string(value: &String) -> anyhow::Result<Save> {
        let bytes = base64::engine::general_purpose::URL_SAFE.decode(value)?;
        // let mut decoder = flate2::read::ZlibDecoder::new(std::io::Cursor::new(z_bytes));
        // let mut bytes: Vec<u8> = Vec::new();
        // decoder.read_to_end(&mut bytes)?;
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
        // let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        // e.write_all(&bytes).unwrap();
        // let c = e.finish().unwrap();
        base64::engine::general_purpose::URL_SAFE.encode(bytes)
    }
}
