use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(PartialEq, Deserialize)]
pub struct Creature {
    pub class: String,
    pub family: String,
    pub creature: String,
    pub trait_name: String,
    pub trait_description: String,
    pub material_name: String,
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
