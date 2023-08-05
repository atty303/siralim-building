use std::io::Cursor;

use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

use data::effect::Effect;
use data::keyword::Keyword;
use data::personality::Personality;
use data::r#trait::{TraitsIndex, TraitsMap};
use data::spell_property::SpellProperty;
use data::Data;

#[derive(RustEmbed, Clone)]
#[folder = "embed/avro/"]
pub struct EmbedAvro;

pub static TRAITS_MAP: Lazy<TraitsMap> = Lazy::new(|| {
    let cursor = Cursor::new(
        EmbedAvro::get("traits.avro")
            .expect("traits.avro not found")
            .data,
    );
    TraitsMap::new(cursor).expect("Failed to load traits")
});

pub static TRAITS_INDEX: Lazy<TraitsIndex> = Lazy::new(|| TraitsIndex::new(&TRAITS_MAP));

fn load_effects() -> Vec<Effect> {
    let reader =
        apache_avro::Reader::new(Cursor::new(EmbedAvro::get("effects.avro").unwrap().data))
            .unwrap();
    let mut effects = Vec::new();
    for value in reader {
        let r = apache_avro::from_value::<Effect>(&value.unwrap()).unwrap();
        effects.push(r);
    }
    effects
}

fn load_spell_properties() -> Vec<SpellProperty> {
    let reader = apache_avro::Reader::new(Cursor::new(
        EmbedAvro::get("spell_properties.avro").unwrap().data,
    ))
    .unwrap();
    let mut xs = Vec::new();
    for value in reader {
        let r = apache_avro::from_value::<SpellProperty>(&value.unwrap()).unwrap();
        xs.push(r);
    }
    xs
}

pub fn load() -> Data {
    Data::from(
        load_effects(),
        Keyword::load(),
        Personality::load(),
        load_spell_properties(),
    )
}
