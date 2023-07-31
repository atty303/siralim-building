use apache_avro::AvroSchema;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Spell {
    pub id: i16,
    pub name: IString,
    pub class: IString,
    pub charges: u8,
    pub source: IString,
    pub description: Vec<IString>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct SpellAvro {
    pub id: i16,
    pub name: String,
    pub class: String,
    pub charges: u8,
    pub source: String,
    pub description: Vec<String>,
}
