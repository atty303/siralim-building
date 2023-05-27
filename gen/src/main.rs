extern crate apache_avro;
extern crate csv;
extern crate data;
extern crate regex;
extern crate serde;
extern crate tantivy;

use std::collections::BTreeSet;
use std::hash::Hash;
use std::hash::{BuildHasher, Hasher};
use std::path::Path;

use apache_avro::AvroSchema;
use csv::StringRecord;
use regex::Regex;
use serde::Deserialize;
use tantivy::{doc, Document, Index};

use data::effect::EffectAvro;
use data::keyword::Keyword;
use data::r#trait;
use data::spell::SpellSchema;
use data::spell_property::SpellPropertyAvro;

trait DefaultHash<S: Ord> {
    fn default_hash(&self, seed: usize) -> S;
}

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
        return reader.deserialize().skip(3).map(|r| r.unwrap()).collect();
    }
}
impl DefaultHash<i32> for CompendiumTraitRecord {
    fn default_hash(&self, seed: usize) -> i32 {
        let mut hasher = ahash::RandomState::with_seed(seed).build_hasher();
        self.hash(&mut hasher);
        (hasher.finish() & 0xFFFFF) as i32
    }
}

impl Hash for CompendiumTraitRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let t = format!(
            "{}:{}:{}:{}",
            self.class, self.family, self.creature, self.trait_name
        );
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

#[derive(Debug, Deserialize)]
pub struct ApiSpellRecord {
    pub name: String,
    pub klass: String,
    pub charges: u8,
    pub source: String,
    pub description: String,
}

impl DefaultHash<i16> for ApiSpellRecord {
    fn default_hash(&self, seed: usize) -> i16 {
        let mut hasher = ahash::RandomState::with_seed(seed).build_hasher();
        //let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        let h = hasher.finish();
        h as i16
        // let mut r = (h & 0xFFFF) as i16;
        // r ^= ((h >> 16) & 0xFFFF) as i16;
        // r ^= ((h >> 32) & 0xFFFF) as i16;
        // r ^= ((h >> 48) & 0xFFFF) as i16;
        // r
    }
}

impl Hash for ApiSpellRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let t = format!("{}:{}", self.name, self.klass);
        t.hash(state)
    }
}

fn load_spells() -> Vec<ApiSpellRecord> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("siralim-ultimate-api/app/data/spells.csv")
        .unwrap();
    return reader.deserialize().map(|r| r.unwrap()).collect();
}

#[derive(Debug, Deserialize)]
pub struct ApiTraitRecord {
    pub name: String,
    pub description: String,
    pub material_name: String,
}

fn load_traits() -> Vec<ApiTraitRecord> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("siralim-ultimate-api/app/data/traits.csv")
        .unwrap();
    return reader.deserialize().map(|r| r.unwrap()).collect();
}

fn load_effects() -> Vec<EffectAvro> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("siralim-ultimate-api/app/data/status_effects.csv")
        .unwrap();
    return reader.deserialize().map(|r| r.unwrap()).collect();
}

fn search_hash_seed<S: Ord, T: DefaultHash<S>>(records: &Vec<T>) -> usize {
    let mut seed = 0usize;
    'seed: for s in 0..100000 {
        let mut set = BTreeSet::new();
        for r in records {
            if !set.insert(r.default_hash(s)) {
                continue 'seed;
            }
        }
        seed = s;
        break 'seed;
    }
    if seed == 100000 {
        panic!("seed not found")
    }
    seed
}

fn build_regex(
    spells: &Vec<ApiSpellRecord>,
    effects: &Vec<EffectAvro>,
    keywords: &Vec<Keyword>,
) -> Vec<Regex> {
    let tokens = spells
        .iter()
        .map(|x| x.name.as_str())
        .chain(effects.iter().map(|x| x.name.as_str()))
        .chain(keywords.iter().map(|x| x.name.as_str()))
        .collect::<Vec<&str>>();
    tokens
        .iter()
        .map(|x| Regex::new(format!("\\b({})\\b", x).as_str()).unwrap())
        .collect()
}

fn tokenize_description(text: String, dict: &Vec<Regex>) -> Vec<String> {
    dict.iter()
        .fold(vec![text], |acc, re| {
            acc.iter()
                .flat_map(|x| {
                    if x.starts_with(":") {
                        vec![x.clone()]
                    } else {
                        re.replace(x.as_str(), "|:$1|")
                            .split("|")
                            .filter(|i| !i.is_empty())
                            .map(String::from)
                            .collect::<Vec<String>>()
                    }
                })
                .collect()
        })
        .iter()
        .map(|x| {
            if x.starts_with(":") {
                x[1..].to_string()
            } else {
                x.clone()
            }
        })
        .collect()
}

fn gen_traits() {
    let creatures = ApiCreatureRecord::load();
    let traits = CompendiumTraitRecord::load();
    let _api_traits = load_traits();
    let effects = load_effects();
    let spells = load_spells();
    let keywords = Keyword::load();

    let words = build_regex(&spells, &effects, &keywords);

    let index_dir = Path::new("embed/traits");
    std::fs::remove_dir_all(index_dir).unwrap();
    std::fs::create_dir(Path::new(index_dir)).unwrap();

    let schema = r#trait::TraitSchema::new();
    let index = Index::create_in_dir(index_dir, schema.schema()).unwrap();

    let mut index_writer = index.writer(3_000_000).unwrap();

    let seed = search_hash_seed(&traits);
    println!("using seed: {}", seed);

    traits.iter().enumerate().for_each(|(i, r)| {
        let hash = r.default_hash(seed);
        //println!("{}: {} {:?}", i, hash, r);

        let trait_name = r.trait_name.replace("\n", " ");
        let trait_name = if trait_name == "Click, Click, Boom" {
            String::from("Click, Click, Boom!")
        } else {
            trait_name
        };
        let trait_name = if trait_name == "Sharpnel Blast" {
            String::from("Shrapnel Blast")
        } else {
            trait_name
        };
        let (description, material) =
            // if let Some(api_trait) = api_traits.iter().find(|t| t.name == trait_name) {
            //     (
            //         api_trait.description.clone(),
            //         api_trait.material_name.clone(),
            //     )
            // } else {
            //     println!("not found: {}", trait_name);
                (r.trait_description.clone(), r.material_name.clone())
            // };
            ;
        let mut doc = Document::default();
        doc.add_i64(schema.id(), hash as i64);
        doc.add_text(schema.class(), r.class.clone());
        doc.add_text(schema.family(), r.family.clone());
        doc.add_text(schema.creature(), r.creature.clone());
        doc.add_text(schema.name(), trait_name.clone());
        doc.add_text(schema.material(), material.clone());

        tokenize_description(description.clone(), &words)
            .iter()
            .for_each(|t| {
                doc.add_text(schema.description(), t);
            });

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

        println!("{}: {:?}", i, schema.to_struct(&doc));
        index_writer.add_document(doc).unwrap();
    });

    index_writer.commit().unwrap();
}

fn gen_effects() {
    let schema = EffectAvro::get_schema();
    let file_writer = std::io::BufWriter::new(
        std::fs::File::create(Path::new("embed/avro/effects.avro")).unwrap(),
    );
    let mut writer = apache_avro::Writer::new(&schema, file_writer);

    for r in load_effects() {
        writer.append_ser(r).unwrap();
    }

    writer.flush().unwrap();
}

fn gen_spells() {
    let spells = load_spells();
    let effects = load_effects();
    let keywords = Keyword::load();

    let words = build_regex(&spells, &effects, &keywords);

    let index_dir = Path::new("embed/spells");
    std::fs::remove_dir_all(index_dir).unwrap();
    std::fs::create_dir(Path::new(index_dir)).unwrap();

    let schema = SpellSchema::new();
    let index = Index::create_in_dir(index_dir, schema.schema()).unwrap();

    let mut index_writer = index.writer(3_000_000).unwrap();

    let seed = search_hash_seed(&spells);
    println!("using seed: {}", seed);

    spells.iter().enumerate().for_each(|(i, r)| {
        let hash = r.default_hash(seed);
        println!("{}: {} {:?}", i, hash, r);

        let mut doc = Document::default();
        doc.add_i64(schema.id(), hash as i64);
        doc.add_text(schema.class(), r.klass.clone());
        doc.add_text(schema.name(), r.name.clone());
        doc.add_u64(schema.charges(), r.charges as u64);
        doc.add_text(schema.source(), r.source.clone());

        tokenize_description(r.description.clone(), &words)
            .iter()
            .for_each(|t| {
                doc.add_text(schema.description(), t);
            });

        println!("{}: {:?}", i, schema.to_struct(&doc));
        index_writer.add_document(doc).unwrap();
    });

    index_writer.commit().unwrap();
}

fn load_spell_properties() -> Vec<SpellPropertyAvro> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("gen/data/spell_properties.csv")
        .unwrap();
    return reader.deserialize().map(|r| r.unwrap()).collect();
}

fn gen_spell_properties() {
    let schema = SpellPropertyAvro::get_schema();
    let file_writer = std::io::BufWriter::new(
        std::fs::File::create(Path::new("embed/avro/spell_properties.avro")).unwrap(),
    );
    let mut writer = apache_avro::Writer::new(&schema, file_writer);

    for r in load_spell_properties() {
        writer.append_ser(r).unwrap();
    }

    writer.flush().unwrap();
}

fn main() {
    gen_traits();
    gen_effects();
    gen_spells();
    gen_spell_properties();
}
