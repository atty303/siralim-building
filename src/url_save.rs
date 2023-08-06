use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

use base64::Engine;
use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter};
use gloo_history::History;
use qstring::QString;

use crate::embed_data::TRAITS_MAP;
use crate::state::UrlState;

pub fn write_state<W: Write>(w: &mut W, state: &UrlState) -> anyhow::Result<()> {
    let mut writer = BitWriter::endian(w, bitstream_io::BigEndian);

    writer.write(8, 1)?; // version
    for m in &state.party {
        for t in &m.traits {
            writer.write(20, t.map(|c| c.id).unwrap_or(0))?;
        }
        writer.write(4, 0)?;
    }
    Ok(())
}

pub fn read_state<R: Read>(r: &mut R) -> anyhow::Result<UrlState> {
    let mut state = UrlState::default();
    let mut reader = BitReader::endian(r, bitstream_io::BigEndian);

    let _version: u8 = reader.read(8)?;
    for m in 0..6 {
        for t in 0..3 {
            let id = reader.read(20)?;
            state.party[m].traits[t] = TRAITS_MAP.get(&id);
        }
        let _: u8 = reader.read(4)?;
    }
    Ok(state)
}

pub fn set_to_url(state: &UrlState) {
    let mut bytes = Vec::new();
    write_state(&mut bytes, state).unwrap();

    // log::debug!("save: {:?} bytes", bytes.len());
    let save_string = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    let history = gloo_history::BrowserHistory::new();
    history
        .replace_with_query("", HashMap::from([("s1", save_string)]))
        .unwrap();
}

pub fn get_from_url<'a>() -> Option<UrlState> {
    let location: web_sys::Location = gloo_utils::window().location();
    let qs = QString::from(location.search().unwrap().as_str());
    let r = if let Some(v1) = qs.get("s1") {
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(v1)
            .ok()?;
        read_state(&mut Cursor::new(bytes))
    } else {
        Err(anyhow::anyhow!("no save found"))
    };
    if r.is_err() {
        log::warn!("failed to parse url save: {:?}", r);
    }
    r.ok()
}
