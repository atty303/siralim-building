use data::Data;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
#[prefix = "data/"]
pub struct EmbedData;

impl EmbedData {
    pub fn load() -> Data {
        let file = EmbedData::get("data/creatures.json").unwrap();
        Data {
            creatures: serde_json::from_slice(file.data.as_ref()).unwrap(),
        }
    }
}
