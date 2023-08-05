use rust_embed::RustEmbed;

use data::effect::Effect;
use data::keyword::Keyword;
use data::personality::Personality;
use data::r#trait::Trait;
use data::spell_property::{SpellProperty, SpellPropertyAvro};
use data::Data;

#[derive(RustEmbed, Clone)]
#[folder = "embed/avro/"]
pub struct EmbedAvro;

fn load_traits() -> Vec<Trait> {
    let reader = apache_avro::Reader::new(std::io::Cursor::new(
        EmbedAvro::get("traits.avro").unwrap().data,
    ))
    .unwrap();
    let mut xs = Vec::new();
    for value in reader {
        let r = apache_avro::from_value::<Trait>(&value.unwrap()).unwrap();
        xs.push(r);
    }
    xs
}

fn load_effects() -> Vec<Effect> {
    let reader = apache_avro::Reader::new(std::io::Cursor::new(
        EmbedAvro::get("effects.avro").unwrap().data,
    ))
    .unwrap();
    let mut effects = Vec::new();
    for value in reader {
        let r = apache_avro::from_value::<Effect>(&value.unwrap()).unwrap();
        effects.push(r);
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
    Data::from(
        load_traits(),
        // spells_index,
        load_effects(),
        Keyword::load(),
        Personality::load(),
        load_spell_properties(),
    )
}
