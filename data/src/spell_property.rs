use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SpellProperty {
    pub id: i8,
    pub name: String,
    pub description: String,
    pub material: String,
    pub source: String,
}

impl SpellProperty {
    pub fn from(a: &SpellPropertyAvro) -> SpellProperty {
        Self {
            id: a.id,
            name: a.name.clone(),
            description: a.description.clone(),
            material: a.material.clone(),
            source: a.source.clone(),
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
