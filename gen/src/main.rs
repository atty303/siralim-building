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

fn main() {
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
    for maybe_row in reader.deserialize().skip(2) {
        let row: Record = maybe_row.unwrap();
        println!("{:?}", row);
    }
}
