use apache_avro::AvroSchema;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SpellProperty {
    pub id: i8,
    pub name: IString,
    pub description: IString,
    pub material: IString,
    pub source: IString,
}

impl SpellProperty {
    pub fn from(a: &SpellPropertyAvro) -> SpellProperty {
        Self {
            id: a.id,
            name: IString::from(a.name.clone()),
            description: IString::from(a.description.clone()),
            material: IString::from(a.material.clone()),
            source: IString::from(a.source.clone()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct SpellPropertyAvro {
    pub id: i8,
    pub name: String,
    pub description: String,
    pub material: String,
    pub source: String,
}
