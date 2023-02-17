extern crate implicit_clone;
extern crate serde;
extern crate tantivy;

use std::io::ErrorKind;

use serde::Deserialize;
use tantivy::collector::DocSetCollector;
use tantivy::query::TermQuery;
use tantivy::schema::IndexRecordOption;
use tantivy::{Index, Term};

use r#trait::{Trait, TraitSchema};

pub mod r#trait;

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct Creature {
    pub class: String,
    pub family: String,
    pub creature: String,
    pub trait_name: String,
    pub trait_description: String,
    pub material_name: String,
    pub stats: Option<CreatureStats>,
    pub uid: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize)]
pub struct CreatureStats {
    pub health: i32,
    pub attack: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub speed: i32,
}

#[derive(Debug, Clone)]
pub struct Data {
    traits_index: Index,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.traits_index.schema() == other.traits_index.schema()
    }
}

impl Data {
    pub fn from(traits_index: Index) -> Data {
        Self { traits_index }
    }
    pub fn traits_index(&self) -> Index {
        self.traits_index.clone()
    }

    pub fn get_trait(&self, id: i64) -> anyhow::Result<Trait> {
        let searcher = self.traits_index.reader()?.searcher();
        let schema = TraitSchema::from(self.traits_index.schema());
        let query = TermQuery::new(
            Term::from_field_i64(schema.id(), id),
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
}
