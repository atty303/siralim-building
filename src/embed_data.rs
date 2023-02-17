// use std::fmt::{Debug, Formatter};
// use std::io::{BufWriter, Write};
// use std::ops::Deref;
// use std::path::{Path, PathBuf};
// use std::sync::Arc;
//
use rust_embed::RustEmbed;
// use tantivy::{Directory, TantivyError};
// use tantivy::directory::{
//     AntiCallToken, FileHandle, FileSlice, TerminatingWrite, WatchCallback, WatchCallbackList,
//     WatchHandle, WritePtr,
// };
// use tantivy::directory::error::{DeleteError, OpenReadError, OpenWriteError};
//
use data::Data;

use crate::embed_directory::EmbedDirectory;

#[derive(RustEmbed, Clone)]
#[folder = "embed/traits/"]
pub struct EmbedTraits;

pub fn load() -> Data {
    let index = tantivy::Index::open(EmbedDirectory::new(EmbedTraits)).unwrap();
    Data::from(index)
}
