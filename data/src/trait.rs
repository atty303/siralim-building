use std::collections::BTreeMap;
use std::io::Read;
use std::ops::Deref;

use apache_avro::AvroSchema;
use indicium::simple::{Indexable, SearchIndex};
use serde::{Deserialize, Serialize};
use stat::Stat;

pub type TraitId = i32;
pub type StatValue = u8;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Trait {
    pub id: TraitId,
    pub class: String,
    pub family: String,
    pub creature: String,
    pub trait_name: String,
    pub trait_description: Vec<String>,
    pub material_name: String,
    pub sources: Vec<String>,
    pub sprite: Option<String>,
    pub stats: Option<Stats>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Stats {
    pub health: StatValue,
    pub attack: StatValue,
    pub intelligence: StatValue,
    pub defense: StatValue,
    pub speed: StatValue,
}

impl Trait {
    pub fn stat(&self, stat: Stat) -> Option<StatValue> {
        match stat {
            Stat::Health => self.stats.as_ref().map(|x| x.health),
            Stat::Attack => self.stats.as_ref().map(|x| x.attack),
            Stat::Intelligence => self.stats.as_ref().map(|x| x.intelligence),
            Stat::Defense => self.stats.as_ref().map(|x| x.defense),
            Stat::Speed => self.stats.as_ref().map(|x| x.speed),
        }
    }
}

impl Indexable for Trait {
    fn strings(&self) -> Vec<String> {
        vec![
            self.class.clone(),
            self.family.clone(),
            self.creature.clone(),
            self.trait_name.clone(),
            self.trait_description.join(" "),
            self.material_name.clone(),
            self.sources.join(" "),
        ]
    }
}

pub struct TraitsMap {
    inner: BTreeMap<TraitId, Trait>,
}

impl TraitsMap {
    pub fn new<R: Read>(avro_read: R) -> Result<Self, apache_avro::Error> {
        let reader = apache_avro::Reader::new(avro_read)?;
        let mut map = BTreeMap::new();
        for result in reader {
            let value = result.expect("Failed to read value");
            let r = apache_avro::from_value::<Trait>(&value).expect("Failed to convert value");
            map.insert(r.id, r);
        }
        Ok(TraitsMap { inner: map })
    }
}

impl Deref for TraitsMap {
    type Target = BTreeMap<TraitId, Trait>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct TraitsIndex {
    inner: SearchIndex<TraitId>,
}

impl Deref for TraitsIndex {
    type Target = SearchIndex<TraitId>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TraitsIndex {
    pub fn new(traits: &TraitsMap) -> Self {
        let mut index = SearchIndex::default();
        traits.values().for_each(|t| {
            index.insert(&t.id, t);
        });
        TraitsIndex { inner: index }
    }
}
