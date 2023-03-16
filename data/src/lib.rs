extern crate apache_avro;
extern crate implicit_clone;
extern crate serde;
extern crate tantivy;

use std::collections::HashMap;
use std::io::ErrorKind;

use implicit_clone::unsync::IString;
use tantivy::collector::{DocSetCollector, TopDocs};
use tantivy::query::{QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption;
use tantivy::{Index, Term};

use effect::Effect;
use keyword::Keyword;
use personality::Personality;
use r#trait::{Trait, TraitSchema};
use spell::{Spell, SpellSchema};
use spell_property::SpellProperty;

pub mod effect;
pub mod keyword;
pub mod personality;
pub mod spell;
pub mod spell_property;
pub mod r#trait;

#[derive(Debug, Clone)]
pub struct Data {
    traits_index: Index,
    spell_index: Index,
    pub effects: HashMap<IString, Effect>,
    pub keywords: HashMap<IString, Keyword>,
    pub personalities: Vec<Personality>,
    pub spell_properties: Vec<SpellProperty>,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.traits_index.schema() == other.traits_index.schema()
        //&& self.effects. == other.effects
    }
}

impl Data {
    pub fn from(
        traits_index: Index,
        spell_index: Index,
        effects: Vec<Effect>,
        keywords: Vec<Keyword>,
        personalities: Vec<Personality>,
        spell_properties: Vec<SpellProperty>,
    ) -> Data {
        let map = effects
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>();
        let keywords = keywords
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>();
        Self {
            traits_index,
            spell_index,
            effects: map,
            keywords,
            personalities,
            spell_properties,
        }
    }

    pub fn traits_index(&self) -> Index {
        self.traits_index.clone()
    }

    pub fn spell_index(&self) -> Index {
        self.spell_index.clone()
    }

    pub fn get_trait(&self, id: i32) -> anyhow::Result<Trait> {
        let searcher = self.traits_index.reader()?.searcher();
        let schema = TraitSchema::from(self.traits_index.schema());
        let query = TermQuery::new(
            Term::from_field_i64(schema.id(), id as i64),
            IndexRecordOption::Basic,
        );
        let docs = searcher.search(&query, &DocSetCollector)?;
        if let Some(address) = docs.iter().next() {
            let doc = searcher.doc(address.clone())?;
            Ok(schema.to_struct(&doc))
        } else {
            Err(std::io::Error::from(ErrorKind::NotFound))?
        }
    }

    pub fn search_trait(&self, qs: &str) -> anyhow::Result<Vec<Trait>> {
        let searcher = self.traits_index.reader()?.searcher();
        let schema = TraitSchema::from(self.traits_index.schema());
        let mut query_parser = QueryParser::for_index(
            &self.traits_index,
            vec![schema.creature(), schema.description()],
        );
        query_parser.set_conjunction_by_default();
        let query = query_parser.parse_query(qs)?;
        let docs = searcher.search(&query, &TopDocs::with_limit(10_000))?;
        docs.into_iter()
            .map(|(_score, address)| {
                let doc = searcher.doc(address)?;
                Ok(schema.to_struct(&doc))
            })
            .collect()
    }

    pub fn get_spell(&self, id: i16) -> anyhow::Result<Spell> {
        let searcher = self.spell_index.reader()?.searcher();
        let schema = SpellSchema::from(self.spell_index.schema());
        let query = TermQuery::new(
            Term::from_field_i64(schema.id(), id as i64),
            IndexRecordOption::Basic,
        );
        let docs = searcher.search(&query, &DocSetCollector)?;
        if let Some(address) = docs.iter().next() {
            let doc = searcher.doc(address.clone())?;
            Ok(schema.to_struct(&doc))
        } else {
            Err(std::io::Error::from(ErrorKind::NotFound))?
        }
    }

    pub fn search_spell(&self, qs: &str) -> anyhow::Result<Vec<Spell>> {
        let searcher = self.spell_index.reader()?.searcher();
        let schema = SpellSchema::from(self.spell_index.schema());
        let mut query_parser =
            QueryParser::for_index(&self.spell_index, vec![schema.name(), schema.description()]);
        query_parser.set_conjunction_by_default();
        let query = query_parser.parse_query(qs)?;
        let docs = searcher.search(&query, &TopDocs::with_limit(10_000))?;
        docs.into_iter()
            .map(|(_score, address)| {
                let doc = searcher.doc(address)?;
                Ok(schema.to_struct(&doc))
            })
            .collect()
    }
}
