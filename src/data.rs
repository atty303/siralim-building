use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(PartialEq, Clone, Deserialize)]
pub struct Creature {
    pub class: String,
    pub family: String,
    pub creature: String,
    pub trait_name: String,
    pub trait_description: String,
    pub material_name: String,
    pub stats: Option<CreatureStats>,
}

#[derive(PartialEq, Clone, Deserialize)]
pub struct CreatureStats {
    pub health: i32,
    pub attack: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub speed: i32,
}

#[derive(RustEmbed)]
#[folder = "data/"]
#[prefix = "data/"]
pub struct Data;

impl Data {
    pub fn creatures() -> Vec<Creature> {
        let file = Data::get("data/creatures.json").unwrap();
        return serde_json::from_slice(file.data.as_ref()).unwrap();
    }
}
