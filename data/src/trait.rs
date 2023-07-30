use apache_avro::AvroSchema;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Trait {
    pub id: i32,
    pub class: IString,
    pub family: IString,
    pub creature: IString,
    pub trait_name: IString,
    pub trait_description: Vec<IString>,
    pub material_name: IString,
    pub sources: Vec<IString>,
    pub sprite: Option<IString>,
    pub stats: Option<Stats>,
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Stats {
    pub health: u8,
    pub attack: u8,
    pub intelligence: u8,
    pub defense: u8,
    pub speed: u8,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct TraitAvro {
    pub id: i32,
    pub class: String,
    pub family: String,
    pub creature: String,
    pub trait_name: String,
    pub trait_description: Vec<String>,
    pub material_name: String,
    pub sources: Vec<String>,
    pub sprite: Option<String>,
    pub stats: Option<StatsAvro>,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, AvroSchema)]
pub struct StatsAvro {
    pub health: u8,
    pub attack: u8,
    pub intelligence: u8,
    pub defense: u8,
    pub speed: u8,
}
