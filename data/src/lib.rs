extern crate apache_avro;
extern crate indicium;
extern crate serde;

use std::collections::HashMap;
use std::ops::Deref;

use effect::Effect;
use keyword::Keyword;
use personality::Personality;
// use spell::{Spell, SpellSchema};
use spell_property::SpellProperty;

pub mod effect;
pub mod keyword;
pub mod personality;
pub mod realm;
pub mod spell;
pub mod spell_property;
pub mod stats;
pub mod r#trait;

#[derive(Debug, Clone)]
pub struct Data {
    pub effects: HashMap<String, Effect>,
    pub keywords: HashMap<String, Keyword>,
    pub personalities: Vec<Personality>,
    pub spell_properties: Vec<SpellProperty>,
}

pub struct EffectsData {
    pub inner: HashMap<String, Effect>,
}

impl Deref for EffectsData {
    type Target = HashMap<String, Effect>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Data {
    pub fn from(
        // spell_index: Index,
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
            // spell_index,
            effects: map,
            keywords,
            personalities,
            spell_properties,
        }
    }
}
