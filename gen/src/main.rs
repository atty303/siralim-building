extern crate csv;
extern crate serde;

use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    class: String,
    family: String,
    creature: String,
    trait_name: String,
    trait_description: String,
    material_name: String,
}

#[derive(Debug, Deserialize)]
struct CreatureRecord {
    battle_sprite: String,
    name: String,
    race: String,
    klass: String,
    health: i32,
    attack: i32,
    intelligence: i32,
    defense: i32,
    speed: i32,
    total: i32,
    sources: String,
    r#trait: String,
}

fn compendium_traits() {
    let rep = reqwest::blocking::get("https://docs.google.com/spreadsheets/d/1qvWwf1fNB5jN8bJ8dFGAVzC7scgDCoBO-hglwjTT4iY/gviz/tq?tqx=out:csv&sheet=Traits").unwrap();
    let body = rep.bytes().unwrap();
    //let text = rep.text().unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(std::io::Cursor::new(body));
    reader.set_headers(StringRecord::from(vec![
        "class",
        "family",
        "creature",
        "trait_name",
        "trait_description",
        "material_name",
    ]));
    let traits = reader
        .deserialize()
        .skip(2)
        .map(|r| r.unwrap())
        .collect::<Vec<Record>>();
    for maybe_row in reader.deserialize().skip(2) {
        let row: Record = maybe_row.unwrap();
        println!("{:?}", row);
    }
}

fn load_creatures() -> Vec<CreatureRecord> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("siralim-ultimate-api/app/data/creatures.csv")
        .unwrap();
    reader
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<CreatureRecord>>()
}

fn main() {
    let creatures = load_creatures();
    println!("{:?}", creatures);
}
