extern crate apache_avro;
extern crate csv;
extern crate data;
extern crate regex;
extern crate serde;

use std::collections::BTreeSet;
use std::fs::File;
use std::hash::Hash;
use std::hash::{BuildHasher, Hasher};
use std::io::{BufWriter, Write};
use std::path::Path;

use apache_avro::AvroSchema;
use csv::StringRecord;
use data::effect::Effect;
use regex::Regex;
use serde::Deserialize;

use data::keyword::Keyword;
use data::r#trait::{Stats, Trait, TraitId};
use data::realm::Realm;
use data::spell::Spell;
use data::spell_property::SpellProperty;

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
        let s = seed as u64;
        let mut hasher = ahash::RandomState::with_seeds(s, s, s, s).build_hasher();
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
    // total: i32,
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
        let s = seed as u64;
        let mut hasher = ahash::RandomState::with_seeds(s, s, s, s).build_hasher();
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

fn load_effects() -> Vec<Effect> {
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
    effects: &Vec<Effect>,
    keywords: &Vec<Keyword>,
) -> Vec<Regex> {
    let tokens = spells
        .iter()
        .map(|x| x.name.as_str())
        .chain(effects.iter().map(|x| x.name.as_str()))
        .chain(keywords.iter().map(|x| x.name))
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
    let realms = Realm::load();

    let words = build_regex(&spells, &effects, &keywords);

    let seed = search_hash_seed(&traits);
    println!("using seed: {}", seed);
    assert_eq!(seed, 2);

    let schema = Trait::get_schema();
    let file_writer =
        std::io::BufWriter::new(std::fs::File::create(Path::new("data/avro/traits.avro")).unwrap());
    let mut writer = apache_avro::Writer::new(&schema, file_writer);

    let mut stats_max = Stats {
        health: u8::MIN,
        attack: u8::MIN,
        intelligence: u8::MIN,
        defense: u8::MIN,
        speed: u8::MIN,
    };
    let mut stats_min = Stats {
        health: u8::MAX,
        attack: u8::MAX,
        intelligence: u8::MAX,
        defense: u8::MAX,
        speed: u8::MAX,
    };
    let mut id_set = BTreeSet::new();
    traits.iter().enumerate().for_each(|(_i, r)| {
        let hash = r.default_hash(seed);
        if !id_set.insert(r.default_hash(seed)) {
            panic!("hash collision");
        }
        // println!("{}: {} {:?}", _i, hash, r);

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

        let api_creature = creatures.iter().find(|c| c.name == r.creature);

        let (sprite, sources, stats) = if let Some(c) = api_creature {
            let s = c
                .sources
                .split(",")
                .map(|s| s.trim().to_string())
                .map(|s| {
                    realms
                        .iter()
                        .find(|r| s.contains(&r.god))
                        .map(|r| format!("{} ({})", r.name, s))
                        .unwrap_or(s)
                })
                .collect::<Vec<_>>();
            let stats = Stats {
                health: c.health as u8,
                attack: c.attack as u8,
                intelligence: c.intelligence as u8,
                defense: c.defense as u8,
                speed: c.speed as u8,
            };
            stats_min.health = stats_min.health.min(stats.health);
            stats_max.health = stats_max.health.max(stats.health);
            stats_min.attack = stats_min.attack.min(stats.attack);
            stats_max.attack = stats_max.attack.max(stats.attack);
            stats_min.intelligence = stats_min.intelligence.min(stats.intelligence);
            stats_max.intelligence = stats_max.intelligence.max(stats.intelligence);
            stats_min.defense = stats_min.defense.min(stats.defense);
            stats_max.defense = stats_max.defense.max(stats.defense);
            stats_min.speed = stats_min.speed.min(stats.speed);
            stats_max.speed = stats_max.speed.max(stats.speed);
            (Some(c.battle_sprite.clone()), s, Some(stats))
        } else {
            (None, vec![], None)
        };

        let r = Trait {
            id: hash as TraitId,
            class: r.class.clone(),
            family: r.family.clone(),
            creature: r.creature.clone(),
            trait_name,
            trait_description: tokenize_description(description.clone(), &words),
            material_name: material.clone(),
            sources,
            sprite,
            stats,
        };

        writer.append_ser(r).unwrap();
    });

    writer.flush().unwrap();

    {
        let mut writer = BufWriter::new(File::create("data/src/stats.rs").unwrap());
        let mut write_line = |name, min, max| {
            writer
                .write(
                    format!(
                        "pub const {}: std::ops::Range<u8> = {}..{};\n",
                        name, min, max
                    )
                    .as_bytes(),
                )
                .unwrap()
        };
        write_line("HEALTH_RANGE", stats_min.health, stats_max.health + 1);
        write_line("ATTACK_RANGE", stats_min.attack, stats_max.attack + 1);
        write_line(
            "INTELLIGENCE_RANGE",
            stats_min.intelligence,
            stats_max.intelligence + 1,
        );
        write_line("DEFENSE_RANGE", stats_min.defense, stats_max.defense + 1);
        write_line("SPEED_RANGE", stats_min.speed, stats_max.speed + 1);
    }
}

fn gen_effects() {
    let schema = Effect::get_schema();
    let file_writer = std::io::BufWriter::new(
        std::fs::File::create(Path::new("data/avro/effects.avro")).unwrap(),
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

    let schema = Spell::get_schema();
    let file_writer =
        std::io::BufWriter::new(std::fs::File::create(Path::new("data/avro/spells.avro")).unwrap());
    let mut writer = apache_avro::Writer::new(&schema, file_writer);

    let seed = search_hash_seed(&spells);
    println!("using seed: {}", seed);
    assert_eq!(seed, 14);

    spells.iter().enumerate().for_each(|(_i, r)| {
        let hash = r.default_hash(seed);
        // println!("{}: {} {:?}", i, hash, r);

        let avro = Spell {
            id: hash,
            class: r.klass.clone(),
            name: r.name.clone(),
            charges: r.charges,
            source: r.source.clone(),
            description: tokenize_description(r.description.clone(), &words),
        };
        writer.append_ser(avro).unwrap();
    });

    writer.flush().unwrap();
}

fn load_spell_properties() -> Vec<SpellProperty> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("gen/data/spell_properties.csv")
        .unwrap();
    return reader.deserialize().map(|r| r.unwrap()).collect();
}

fn gen_spell_properties() {
    let schema = SpellProperty::get_schema();
    let file_writer = std::io::BufWriter::new(
        std::fs::File::create(Path::new("data/avro/spell_properties.avro")).unwrap(),
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
