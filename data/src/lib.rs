extern crate apache_avro;
extern crate implicit_clone;
extern crate indicium;
extern crate serde;

use std::collections::{BTreeMap, HashMap};

use indicium::simple::SearchIndex;

use effect::Effect;
use keyword::Keyword;
use personality::Personality;
use r#trait::Trait;
// use spell::{Spell, SpellSchema};
use spell_property::SpellProperty;

pub mod effect;
pub mod keyword;
pub mod personality;
pub mod spell;
pub mod spell_property;
pub mod stats;
pub mod r#trait;

#[derive(Debug, Clone)]
pub struct Data {
    pub traits: BTreeMap<i32, Trait>,
    pub traits_index: SearchIndex<i32>,
    // spell_index: Index,
    pub effects: HashMap<String, Effect>,
    pub keywords: HashMap<String, Keyword>,
    pub personalities: Vec<Personality>,
    pub spell_properties: Vec<SpellProperty>,
}

impl Data {
    pub fn from(
        traits: Vec<Trait>,
        // spell_index: Index,
        effects: Vec<Effect>,
        keywords: Vec<Keyword>,
        personalities: Vec<Personality>,
        spell_properties: Vec<SpellProperty>,
    ) -> Data {
        let mut traits_map = BTreeMap::new();
        let mut traits_index = SearchIndex::default();
        traits.iter().for_each(|t| {
            traits_map.insert(t.id, t.clone());
            traits_index.insert(&t.id, t);
        });

        let map = effects
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>();
        let keywords = keywords
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>();
        Self {
            traits: traits_map,
            traits_index,
            // spell_index,
            effects: map,
            keywords,
            personalities,
            spell_properties,
        }
    }

    // pub fn traits_index(&self) -> Index {
    //     self.traits_index.clone()
    // }
    //
    // pub fn spell_index(&self) -> Index {
    //     self.spell_index.clone()
    // }
    //
    // pub fn get_trait(&self, id: i32) -> anyhow::Result<Trait> {
    //     let searcher = self.traits_index.reader()?.searcher();
    //     let schema = TraitSchema::from(self.traits_index.schema());
    //     let query = TermQuery::new(
    //         Term::from_field_i64(schema.id(), id as i64),
    //         IndexRecordOption::Basic,
    //     );
    //     let docs = searcher.search(&query, &DocSetCollector)?;
    //     if let Some(address) = docs.iter().next() {
    //         let doc = searcher.doc(address.clone())?;
    //         Ok(schema.to_struct(&doc))
    //     } else {
    //         Err(std::io::Error::from(ErrorKind::NotFound))?
    //     }
    // }

    // pub fn search_trait(&self, qs: &str) -> anyhow::Result<Vec<Trait>> {
    //     let searcher = self.traits_index.reader()?.searcher();
    //     let schema = TraitSchema::from(self.traits_index.schema());
    //     let mut query_parser = QueryParser::for_index(
    //         &self.traits_index,
    //         vec![schema.creature(), schema.description()],
    //     );
    //     query_parser.set_conjunction_by_default();
    //     let query = query_parser.parse_query(qs)?;
    //     let docs = searcher.search(&query, &TopDocs::with_limit(10_000))?;
    //     docs.into_iter()
    //         .map(|(_score, address)| {
    //             let doc = searcher.doc(address)?;
    //             Ok(schema.to_struct(&doc))
    //         })
    //         .collect()
    // }
    //
    // pub fn get_spell(&self, id: i16) -> anyhow::Result<Spell> {
    //     let searcher = self.spell_index.reader()?.searcher();
    //     let schema = SpellSchema::from(self.spell_index.schema());
    //     let query = TermQuery::new(
    //         Term::from_field_i64(schema.id(), id as i64),
    //         IndexRecordOption::Basic,
    //     );
    //     let docs = searcher.search(&query, &DocSetCollector)?;
    //     if let Some(address) = docs.iter().next() {
    //         let doc = searcher.doc(address.clone())?;
    //         Ok(schema.to_struct(&doc))
    //     } else {
    //         Err(std::io::Error::from(ErrorKind::NotFound))?
    //     }
    // }
    //
    // pub fn search_spell(&self, qs: &str) -> anyhow::Result<Vec<Spell>> {
    //     let searcher = self.spell_index.reader()?.searcher();
    //     let schema = SpellSchema::from(self.spell_index.schema());
    //     let mut query_parser =
    //         QueryParser::for_index(&self.spell_index, vec![schema.name(), schema.description()]);
    //     query_parser.set_conjunction_by_default();
    //     let query = query_parser.parse_query(qs)?;
    //     let docs = searcher.search(&query, &TopDocs::with_limit(10_000))?;
    //     docs.into_iter()
    //         .map(|(_score, address)| {
    //             let doc = searcher.doc(address)?;
    //             Ok(schema.to_struct(&doc))
    //         })
    //         .collect()
    // }
}
