use data::Data;
use rust_embed::RustEmbed;

use crate::embed_directory::EmbedDirectory;

#[derive(RustEmbed, Clone)]
#[folder = "embed/traits/"]
pub struct EmbedTraits;

pub fn load() -> Data {
    let index = tantivy::Index::open(EmbedDirectory::new(EmbedTraits)).unwrap();
    Data::from(index)
}
