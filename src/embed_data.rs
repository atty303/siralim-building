use data::effect::{Effect, EffectAvro};
use data::keyword::Keyword;
use data::personality::Personality;
use data::spell_property::{SpellProperty, SpellPropertyAvro};
use data::Data;
use rust_embed::RustEmbed;

use crate::embed_directory::EmbedDirectory;

#[derive(RustEmbed, Clone)]
#[folder = "embed/avro/"]
pub struct EmbedAvro;

#[derive(RustEmbed, Clone)]
#[folder = "embed/traits/"]
pub struct EmbedTraits;

#[derive(RustEmbed, Clone)]
#[folder = "embed/spells/"]
pub struct EmbedSpells;

fn load_effects() -> Vec<Effect> {
    let reader = apache_avro::Reader::new(std::io::Cursor::new(
        EmbedAvro::get("effects.avro").unwrap().data,
    ))
    .unwrap();
    let mut effects = Vec::new();
    for value in reader {
        let r = apache_avro::from_value::<EffectAvro>(&value.unwrap()).unwrap();
        effects.push(Effect::from(&r));
    }
    effects
}

fn load_spell_properties() -> Vec<SpellProperty> {
    let reader = apache_avro::Reader::new(std::io::Cursor::new(
        EmbedAvro::get("spell_properties.avro").unwrap().data,
    ))
    .unwrap();
    let mut xs = Vec::new();
    for value in reader {
        let r = apache_avro::from_value::<SpellPropertyAvro>(&value.unwrap()).unwrap();
        xs.push(SpellProperty::from(&r));
    }
    xs
}

pub fn load() -> Data {
    let traits_index = tantivy::Index::open(EmbedDirectory::new(EmbedTraits)).unwrap();
    let spells_index = tantivy::Index::open(EmbedDirectory::new(EmbedSpells)).unwrap();

    Data::from(
        traits_index,
        spells_index,
        load_effects(),
        Keyword::load(),
        Personality::load(),
        load_spell_properties(),
    )
}
