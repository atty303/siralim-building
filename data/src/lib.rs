pub mod r#trait;

extern crate implicit_clone;
extern crate serde;
extern crate tantivy;

use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct Creature {
    pub class: String,
    pub family: String,
    pub creature: String,
    pub trait_name: String,
    pub trait_description: String,
    pub material_name: String,
    pub stats: Option<CreatureStats>,
    pub uid: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct CreatureStats {
    pub health: i32,
    pub attack: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub speed: i32,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Data {
    pub creatures: Vec<Creature>,
}

impl Data {
    pub fn get_creature_by_uid(&self, uid: &String) -> Option<&Creature> {
        self.creatures.iter().find(|c| &c.uid == uid)
    }
}
