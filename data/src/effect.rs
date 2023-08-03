use apache_avro::AvroSchema;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

// TODO: replace with EffectAvro
#[derive(Debug, Clone)]
pub struct Effect {
    pub name: String,
    pub category: String,
    pub turns: Option<u8>,
    pub leave_chance: Option<u8>,
    pub max_stacks: u8,
    pub icon: String,
    pub description: String,
}

impl Effect {
    pub fn from(a: &EffectAvro) -> Effect {
        Self {
            name: String::from(a.name.clone()),
            category: String::from(a.category.clone()),
            turns: a.turns,
            leave_chance: a.leave_chance,
            max_stacks: a.max_stacks,
            icon: String::from(a.icon.clone()),
            description: String::from(a.description.clone()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct EffectAvro {
    pub name: String,
    pub category: String,
    pub turns: Option<u8>,
    pub leave_chance: Option<u8>,
    pub max_stacks: u8,
    pub icon: String,
    pub description: String,
}
