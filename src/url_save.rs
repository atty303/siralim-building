use std::io::Cursor;

use base64::Engine;
use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter};
use qstring::QString;

use crate::embed_data::TRAITS_MAP;
use crate::state::UrlState;

#[derive(Debug)]
pub struct UrlSave {
    bytes: Vec<u8>,
}

impl UrlSave {
    pub fn from_state(state: &UrlState) -> anyhow::Result<UrlSave> {
        let mut bytes: Vec<u8> = Vec::new();
        {
            let mut writer = BitWriter::endian(&mut bytes, bitstream_io::BigEndian);
            for m in &state.party {
                for t in &m.traits {
                    writer.write(20, t.map(|c| c.id).unwrap_or(0))?;
                    writer.write(4, 0)?;
                }
            }
        }
        Ok(UrlSave { bytes })
    }

    pub fn to_state<'a>(&self) -> UrlState<'a> {
        let mut state = UrlState::default();

        let mut cursor = Cursor::new(&self.bytes);
        {
            let mut reader = BitReader::endian(&mut cursor, bitstream_io::BigEndian);
            for m in 0..6 {
                for t in 0..3 {
                    let id = reader.read(20).unwrap();
                    let _: u8 = reader.read(4).unwrap();
                    state.party[m].traits[t] = TRAITS_MAP.get(&id);
                }
            }
        }

        state
    }

    pub fn from_string(value: &str) -> anyhow::Result<UrlSave> {
        let bytes = base64::engine::general_purpose::URL_SAFE.decode(value)?;
        // let mut decoder = flate2::read::ZlibDecoder::new(std::io::Cursor::new(z_bytes));
        // let mut bytes: Vec<u8> = Vec::new();
        // decoder.read_to_end(&mut bytes)?;
        Ok(UrlSave { bytes })
    }

    pub fn to_string(&self) -> String {
        log::debug!("save: {:?} bytes", self.bytes.len());
        // let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        // e.write_all(&bytes).unwrap();
        // let c = e.finish().unwrap();
        base64::engine::general_purpose::URL_SAFE.encode(&self.bytes)
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
        let r = if let Some(v1) = qs.get("s1") {
            UrlSave::from_string(v1)
        } else {
            Err(anyhow::anyhow!("no save found"))
        };
        if r.is_err() {
            log::warn!("failed to parse url save: {:?}", r);
        }
        r.ok()
    }
}
