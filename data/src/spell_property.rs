use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct SpellProperty {
    pub id: i8,
    pub name: String,
    pub description: String,
    pub material: String,
    pub source: String,
}
