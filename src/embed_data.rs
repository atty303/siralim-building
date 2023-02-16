use data::Data;
use rust_embed::{EmbeddedFile, RustEmbed};
use std::fmt::{Debug, Formatter};
use std::io::{BufWriter, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tantivy::directory::error::{DeleteError, OpenReadError, OpenWriteError};
use tantivy::directory::{
    AntiCallToken, FileHandle, FileSlice, TerminatingWrite, WatchCallback, WatchCallbackList,
    WatchHandle, WritePtr,
};
use tantivy::{Directory, TantivyError};

#[derive(RustEmbed)]
#[folder = "data/"]
#[prefix = "data/"]
pub struct EmbedData;

#[derive(RustEmbed, Clone)]
#[folder = "embed/traits/"]
pub struct EmbedTraits;

impl EmbedData {
    pub fn load() -> Data {
        let file = EmbedData::get("data/creatures.json").unwrap();
        Data {
            creatures: serde_json::from_slice(file.data.as_ref()).unwrap(),
        }
    }
}
