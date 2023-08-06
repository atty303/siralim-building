use std::io::Cursor;

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

use data::effect::EffectsMap;
use data::keyword::KeywordsMap;
use data::r#trait::{TraitsIndex, TraitsMap};

#[derive(RustEmbed, Clone)]
#[folder = "data/avro/"]
pub struct DataAvro;

pub static TRAITS_MAP: Lazy<TraitsMap> = Lazy::new(|| {
    let cursor = Cursor::new(
        DataAvro::get("traits.avro")
            .expect("traits.avro not found")
            .data,
    );
    TraitsMap::new(cursor).expect("Failed to load traits")
});

pub static TRAITS_INDEX: Lazy<TraitsIndex> = Lazy::new(|| TraitsIndex::new(&TRAITS_MAP));

pub static EFFECTS_MAP: Lazy<EffectsMap> = Lazy::new(|| {
    let cursor = Cursor::new(
        DataAvro::get("effects.avro")
            .expect("effects.avro not found")
            .data,
    );
    EffectsMap::new(cursor).expect("Failed to load effects")
});

pub static KEYWORDS_MAP: Lazy<KeywordsMap> = Lazy::new(|| KeywordsMap::new());

// pub static SPELL_PROPERTIES_MAP: Lazy<SpellPropertiesMap> = Lazy::new(|| {
//     let cursor = Cursor::new(
//         EmbedAvro::get("spell_properties.avro")
//             .expect("spell_properties.avro not found")
//             .data,
//     );
//     SpellPropertiesMap::new(cursor).expect("Failed to load spell properties")
// });
