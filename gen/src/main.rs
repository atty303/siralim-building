extern crate csv;
extern crate data;
extern crate serde;
extern crate tantivy;

use std::collections::BTreeSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;

use csv::StringRecord;
use serde::Deserialize;
use tantivy::{doc, Document, Index};

use data::r#trait;

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

    let index_dir = Path::new("embed/traits");
    std::fs::remove_dir_all(index_dir).unwrap();
    std::fs::create_dir(Path::new(index_dir)).unwrap();

    let schema = r#trait::TraitSchema::new();
    let index = Index::create_in_dir(index_dir, schema.schema()).unwrap();

    let mut index_writer = index.writer(3_000_000).unwrap();

    let mut hash_set = BTreeSet::new();

    traits.iter().enumerate().for_each(|(i, r)| {
        let hash = r.default_hash();
        if !hash_set.insert(hash) {
            panic!("hash collided at {}", i);
        }
        println!("{}: {}", i, hash);

        let mut doc = Document::default();
        doc.add_i64(schema.id(), hash as i64);
        doc.add_text(schema.class(), r.class.clone());
        doc.add_text(schema.family(), r.family.clone());
        doc.add_text(schema.creature(), r.creature.clone());
        doc.add_text(schema.name(), r.trait_name.clone());
        doc.add_text(schema.description(), r.trait_description.clone());
        doc.add_text(schema.material(), r.material_name.clone());

        let api_creature = creatures.iter().find(|c| c.name == r.creature);
        if let Some(c) = api_creature {
            doc.add_text(schema.sprite(), c.battle_sprite.clone());
            doc.add_u64(schema.health(), c.health as u64);
            doc.add_u64(schema.attack(), c.attack as u64);
            doc.add_u64(schema.intelligence(), c.intelligence as u64);
            doc.add_u64(schema.defense(), c.defense as u64);
            doc.add_u64(schema.speed(), c.speed as u64);
            doc.add_u64(schema.total(), c.total as u64);

            for s in c.sources.split(",") {
                doc.add_text(schema.sources(), s.trim().to_string())
            }
        }

        index_writer.add_document(doc).unwrap();
    });

    index_writer.commit().unwrap();
}

fn main() {
    gen_traits();
}
