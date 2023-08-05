use std::collections::HashMap;
use std::io::Read;
use std::ops::Deref;

use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Effect {
    pub name: String,
    pub category: String,
    pub turns: Option<u8>,
    pub leave_chance: Option<u8>,
    pub max_stacks: u8,
    pub icon: String,
    pub description: String,
}

pub struct EffectsMap {
    inner: HashMap<String, Effect>,
}

impl Deref for EffectsMap {
    type Target = HashMap<String, Effect>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl EffectsMap {
    pub fn new<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut effects = HashMap::new();
        for result in reader {
            let value = &result.expect("Failed to read value");
            let r = apache_avro::from_value::<Effect>(&value).expect("Failed to convert value");
            effects.insert(r.name.clone(), r);
        }
        Ok(EffectsMap { inner: effects })
    }
}
