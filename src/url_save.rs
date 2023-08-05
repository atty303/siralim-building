use std::io::Cursor;

use apache_avro::AvroSchema;
use base64::Engine;
use qstring::QString;
use serde::{Deserialize, Serialize};

use crate::embed_data::TRAITS_MAP;
use crate::state::{Member, UrlState};
use data::r#trait::TraitId;

#[derive(Serialize, Deserialize, AvroSchema, Debug)]
pub struct UrlSave {
    party: Vec<SaveMember>,
}

#[derive(Serialize, Deserialize, AvroSchema, Debug)]
struct SaveMember {
    traits: Vec<Option<TraitId>>,
}

impl UrlSave {
    pub fn from_state(state: &UrlState) -> anyhow::Result<UrlSave> {
        Ok(UrlSave {
            party: state
                .party
                .iter()
                .map(|m| SaveMember {
                    traits: m.traits.iter().map(|t| t.clone().map(|c| c.id)).collect(),
                })
                .collect(),
        })
    }

    pub fn to_state<'a>(&self) -> UrlState<'a> {
        UrlState {
            party: self
                .party
                .iter()
                .map(|m| Member {
                    traits: m
                        .traits
                        .iter()
                        .map(|t| match t {
                            Some(id) => TRAITS_MAP.get(id),
                            None => None,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    pub fn from_string(value: &str) -> anyhow::Result<UrlSave> {
        let bytes = base64::engine::general_purpose::URL_SAFE.decode(value)?;
        // let mut decoder = flate2::read::ZlibDecoder::new(std::io::Cursor::new(z_bytes));
        // let mut bytes: Vec<u8> = Vec::new();
        // decoder.read_to_end(&mut bytes)?;
        let avro_value =
            apache_avro::from_avro_datum(&UrlSave::get_schema(), &mut Cursor::new(bytes), None)?;
        let save: UrlSave = apache_avro::from_value(&avro_value)?;
        Ok(save)
    }

    pub fn to_string(&self) -> String {
        let save_value = apache_avro::to_value(self).unwrap();
        let bytes = apache_avro::to_avro_datum(&UrlSave::get_schema(), save_value).unwrap();
        // let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        // e.write_all(&bytes).unwrap();
        // let c = e.finish().unwrap();
        base64::engine::general_purpose::URL_SAFE.encode(bytes)
    }

    pub fn set_to_url(&self) {
        let location: web_sys::Location = web_sys::window().unwrap().location();
        let history: web_sys::History = web_sys::window().unwrap().history().unwrap();
        let save_string = self.to_string();
        history
            .replace_state_with_url(
                &wasm_bindgen::JsValue::null(),
                "",
                Some(format!("{}?s1={}", location.pathname().unwrap(), save_string).as_str()),
            )
            .unwrap();
    }

    pub fn get_from_url() -> Option<UrlSave> {
        let location: web_sys::Location = web_sys::window().unwrap().location();
        let qs = QString::from(location.search().unwrap().as_str());
        if let Some(v1) = qs.get("s1") {
            UrlSave::from_string(v1).ok()
        } else {
            None
        }
    }
}
