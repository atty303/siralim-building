use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Spell {
    pub id: i16,
    pub name: String,
    pub class: String,
    pub charges: u8,
    pub source: String,
    pub description: Vec<String>,
}
