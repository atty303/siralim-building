extern crate csv;
extern crate serde;
extern crate tantivy;

use csv::StringRecord;
use serde::Deserialize;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{STORED, TEXT};

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

fn load_compendium_traits() -> Vec<Record> {
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
    traits
    // for maybe_row in reader.deserialize().skip(2) {
    //     let row: Record = maybe_row.unwrap();
    //     println!("{:?}", row);
    // }
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

fn gen_traits() {
    let creatures = load_creatures();
    let traits = load_compendium_traits();

    let a = traits.iter().map(|t| {
        let r = creatures.iter().find(|c| c.name == t.creature);
        (t, r)
    });

    let mut schema_builder = tantivy::schema::Schema::builder();
    let creature = schema_builder.add_text_field("creature", TEXT | STORED);
    let trait_description = schema_builder.add_text_field("trait_description", TEXT | STORED);
    let schema = schema_builder.build();

    let index = tantivy::Index::create_in_dir("embed/traits", schema).unwrap();

    let mut index_writer = index.writer(100_000_000).unwrap();

    a.for_each(|r| {
        index_writer
            .add_document(doc!(
                creature => r.0.creature.clone(),
                trait_description => r.0.trait_description.clone(),
            ))
            .unwrap();
    });

    index_writer.commit().unwrap();
}

fn main() {
    let mut schema_builder = tantivy::schema::Schema::builder();
    let creature = schema_builder.add_text_field("creature", TEXT | STORED);
    let trait_description = schema_builder.add_text_field("trait_description", TEXT | STORED);
    let schema = schema_builder.build();

    //    let index = tantivy::Index::create_in_dir("embed/traits", schema.clone()).unwrap();
    let index = tantivy::Index::open_in_dir("embed/traits").unwrap();

    let reader = index.reader().unwrap();
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![creature, trait_description]);
    let query = query_parser.parse_query("attack").unwrap();

    let docs = searcher
        .search(&query, &tantivy::collector::TopDocs::with_limit(10))
        .unwrap();

    for (_score, doc_address) in docs {
        let doc = searcher.doc(doc_address).unwrap();
        println!("{}", schema.to_json(&doc));
    }
}
