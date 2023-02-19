extern crate apache_avro;
extern crate implicit_clone;
extern crate serde;
extern crate tantivy;

use std::collections::HashMap;
use std::io::ErrorKind;
use std::iter::FromIterator;

use implicit_clone::unsync::IString;
use tantivy::collector::{DocSetCollector, TopDocs};
use tantivy::query::{QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption;
use tantivy::{Index, Term};

use effect::Effect;
use r#trait::{Trait, TraitSchema};

pub mod effect;
pub mod r#trait;

#[derive(Debug, Clone)]
pub struct Data {
    traits_index: Index,
    pub effects: HashMap<IString, Effect>,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.traits_index.schema() == other.traits_index.schema()
        //&& self.effects. == other.effects
    }
}

impl Data {
    pub fn from(traits_index: Index, effects: Vec<Effect>) -> Data {
        let map = effects
            .iter()
            .map(|e| (e.name.clone(), e.clone()))
            .collect::<HashMap<_, _>>();
        Self {
            traits_index,
            effects: map,
        }
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

    pub fn search_trait(&self, qs: &str) -> anyhow::Result<Vec<Trait>> {
        let searcher = self.traits_index.reader()?.searcher();
        let schema = TraitSchema::from(self.traits_index.schema());
        let query_parser = QueryParser::for_index(&self.traits_index, vec![schema.description()]);
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
