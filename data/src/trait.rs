use apache_avro::AvroSchema;
use indicium::simple::Indexable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Trait {
    pub id: i32,
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

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, AvroSchema)]
pub struct Stats {
    pub health: u8,
    pub attack: u8,
    pub intelligence: u8,
    pub defense: u8,
    pub speed: u8,
}

impl Trait {
    pub fn health(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.health)
    }
    pub fn attack(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.attack)
    }
    pub fn intelligence(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.intelligence)
    }
    pub fn defense(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.defense)
    }
    pub fn speed(&self) -> Option<u8> {
        self.stats.as_ref().map(|x| x.speed)
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
