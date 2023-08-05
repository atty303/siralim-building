use apache_avro::AvroSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Read;
use std::ops::Deref;

pub type SpellPropertyId = i8;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct SpellProperty {
    pub id: SpellPropertyId,
    pub name: String,
    pub description: String,
    pub material: String,
    pub source: String,
}

pub struct SpellPropertiesMap {
    inner: BTreeMap<SpellPropertyId, SpellProperty>,
}

impl SpellPropertiesMap {
    pub fn new<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut map = BTreeMap::new();
        for result in reader {
            let value = result.expect("Failed to read value");
            let r =
                apache_avro::from_value::<SpellProperty>(&value).expect("Failed to convert value");
            map.insert(r.id, r);
        }
        Ok(SpellPropertiesMap { inner: map })
    }
}

impl Deref for SpellPropertiesMap {
    type Target = BTreeMap<SpellPropertyId, SpellProperty>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
