extern crate csv;
extern crate serde;
extern crate tantivy;

use std::collections::BTreeSet;
use std::hash::Hash;
use std::hash::{BuildHasher, Hasher};
use std::path::Path;

use csv::StringRecord;
use serde::Deserialize;
use tantivy::query::{AllQuery, QueryParser, RangeQuery, TermQuery, TermSetQuery};
use tantivy::schema::{
    Cardinality, IndexRecordOption, NumericOptions, Schema, TextOptions, FAST, INDEXED, STORED,
    STRING, TEXT,
};
use tantivy::{doc, Document, Term};

#[derive(Debug, Deserialize)]
struct CompendiumTraitRecord {
    class: String,
    family: String,
    creature: String,
    trait_name: String,
    trait_description: String,
    material_name: String,
}

impl CompendiumTraitRecord {
    fn load() -> Vec<CompendiumTraitRecord> {
        let rep = reqwest::blocking::get("https://docs.google.com/spreadsheets/d/1qvWwf1fNB5jN8bJ8dFGAVzC7scgDCoBO-hglwjTT4iY/gviz/tq?tqx=out:csv&sheet=Traits").unwrap();
        let body = rep.bytes().unwrap();
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
        return reader.deserialize().skip(2).map(|r| r.unwrap()).collect();
    }

    fn default_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for CompendiumTraitRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let t = format!(
            "{}:{}:{}:{}:{}",
            self.class, self.family, self.creature, self.trait_name, self.material_name
        );
        // self.class.as_str().hash(state);
        // self.family.as_str().hash(state);
        // self.creature.as_str().hash(state);
        // self.trait_name.as_str().hash(state);
        // self.material_name.as_str().hash(state);
        t.hash(state)
    }
}

#[derive(Debug, Deserialize)]
struct ApiCreatureRecord {
    battle_sprite: String,
    name: String,
    // race: String,
    // klass: String,
    health: i32,
    attack: i32,
    intelligence: i32,
    defense: i32,
    speed: i32,
    total: i32,
    sources: String,
    // r#trait: String,
}

impl ApiCreatureRecord {
    fn load() -> Vec<ApiCreatureRecord> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path("siralim-ultimate-api/app/data/creatures.csv")
            .unwrap();
        return reader.deserialize().map(|r| r.unwrap()).collect();
    }
}

fn gen_traits() {
    let creatures = ApiCreatureRecord::load();
    let traits = CompendiumTraitRecord::load();

    let all = traits.iter().map(|t| {
        let r = creatures.iter().find(|c| c.name == t.creature);
        (t, r)
    });

    let mut schema_builder = Schema::builder();
    let id = schema_builder.add_u64_field("id", INDEXED | STORED);
    let class = schema_builder.add_text_field("class", TEXT | STORED | FAST);
    let family = schema_builder.add_text_field("family", TEXT | STORED);
    let creature = schema_builder.add_text_field("creature", TEXT | STORED);
    let name = schema_builder.add_text_field("name", TEXT | STORED);
    let description = schema_builder.add_text_field("description", TEXT | STORED);
    let material = schema_builder.add_text_field("material", TEXT | STORED);
    let sources = schema_builder.add_text_field("sources", TextOptions::default().set_stored());
    let sprite = schema_builder.add_text_field("sprite", TextOptions::default().set_stored());
    let health = schema_builder.add_u64_field("health", STORED);
    let attack = schema_builder.add_u64_field("attack", STORED);
    let intelligence = schema_builder.add_u64_field("intelligence", STORED);
    let defense = schema_builder.add_u64_field("defense", STORED);
    let speed = schema_builder.add_u64_field("speed", STORED);
    let total = schema_builder.add_u64_field("total", STORED);

    let schema = schema_builder.build();

    std::fs::remove_dir_all(Path::new("embed/traits")).unwrap();
    std::fs::create_dir(Path::new("embed/traits")).unwrap();
    let index = tantivy::Index::create_in_dir("embed/traits", schema).unwrap();

    let mut index_writer = index.writer(3_000_000).unwrap();

    let mut hash_set = BTreeSet::new();
    all.enumerate().for_each(|(i, r)| {
        let hash = r.0.default_hash();
        if !hash_set.insert(hash) {
            panic!("hash collided at {}", i);
        }

        let mut doc = Document::default();
        doc.add_u64(id, hash);
        doc.add_text(class, r.0.class.clone());
        doc.add_text(family, r.0.family.clone());
        doc.add_text(creature, r.0.creature.clone());
        doc.add_text(name, r.0.trait_name.clone());
        doc.add_text(description, r.0.trait_description.clone());
        doc.add_text(material, r.0.material_name.clone());

        if let Some(c) = r.1 {
            doc.add_text(sprite, c.battle_sprite.clone());
            doc.add_u64(health, c.health as u64);
            doc.add_u64(attack, c.attack as u64);
            doc.add_u64(intelligence, c.intelligence as u64);
            doc.add_u64(defense, c.defense as u64);
            doc.add_u64(speed, c.speed as u64);
            doc.add_u64(total, c.total as u64);

            for s in c.sources.split(",") {
                doc.add_text(sources, s.trim().to_string())
            }
        }
        index_writer.add_document(doc).unwrap();
    });

    index_writer.commit().unwrap();
}

fn main() {
    gen_traits();

    let index = tantivy::Index::open_in_dir("embed/traits").unwrap();

    let reader = index.reader().unwrap();
    let searcher = reader.searcher();

    // let query = TermSetQuery::new(vec![
    //     Term::from_field_u64(
    //         index.schema().get_field("id").unwrap(),
    //         13421718727927563020u64,
    //     ),
    //     Term::from_field_u64(
    //         index.schema().get_field("id").unwrap(),
    //         2016719850152869480u64,
    //     ),
    // ]);
    let query = AllQuery;
    let docs = searcher
        .search(&query, &tantivy::collector::DocSetCollector)
        .unwrap();
    for doc_address in docs {
        let doc = searcher.doc(doc_address).unwrap();
        let nd = index.schema().to_named_doc(&doc);
        println!(
            "{:?}: {:?}",
            nd.0.get("id").unwrap().get(0).unwrap(),
            nd.0.get("health")
        );
        //println!("{}", index.schema().to_json(&doc));
    }
}
