use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Effect {
    pub name: String,
    pub category: String,
    pub turns: Option<u8>,
    pub leave_chance: Option<u8>,
    pub max_stacks: u8,
    pub icon: String,
    pub description: String,
}
