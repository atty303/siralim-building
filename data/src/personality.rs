use std::collections::BTreeMap;
use std::ops::Deref;

use stat::Stat;

pub type PersonalityId = i8;

#[derive(Debug, Clone)]
pub struct Personality {
    pub id: PersonalityId,
    pub name: &'static str,
    pub positive: Stat,
    pub negative: Stat,
}

impl Personality {
    fn new(id: i8, name: &'static str, positive: Stat, negative: Stat) -> Personality {
        Self {
            id,
            name,
            positive,
            negative,
        }
    }

    pub fn load() -> Vec<Personality> {
        let xs: Vec<(&'static str, Stat, Stat)> = vec![
            ("Analytical", Stat::Intelligence, Stat::Health),
            ("Apathetic", Stat::Health, Stat::Defense),
            ("Bashful", Stat::Speed, Stat::Attack),
            ("Brave", Stat::Attack, Stat::Defense),
            ("Brutal", Stat::Attack, Stat::Intelligence),
            ("Careful", Stat::Intelligence, Stat::Attack),
            ("Clever", Stat::Intelligence, Stat::Defense),
            ("Daring", Stat::Attack, Stat::Speed),
            ("Gentle", Stat::Defense, Stat::Intelligence),
            ("Indifferent", Stat::Health, Stat::Intelligence),
            ("Lazy", Stat::Health, Stat::Attack),
            ("Nervous", Stat::Speed, Stat::Health),
            ("Peaceful", Stat::Defense, Stat::Speed),
            ("Protective", Stat::Defense, Stat::Attack),
            ("Reckless", Stat::Attack, Stat::Health),
            ("Relaxed", Stat::Health, Stat::Speed),
            ("Selfless", Stat::Defense, Stat::Health),
            ("Shrewd", Stat::Intelligence, Stat::Speed),
            ("Shy", Stat::Speed, Stat::Intelligence),
            ("Timid", Stat::Speed, Stat::Defense),
        ];
        xs.into_iter()
            .enumerate()
            .map(|(i, x)| Personality::new((i + 1) as PersonalityId, x.0, x.1, x.2))
            .collect()
    }
}

pub struct PersonalitiesMap {
    inner: BTreeMap<PersonalityId, Personality>,
}

impl PersonalitiesMap {
    pub fn new() -> PersonalitiesMap {
        let mut inner = BTreeMap::new();
        for x in Personality::load() {
            inner.insert(x.id, x);
        }
        PersonalitiesMap { inner }
    }
}

impl Deref for PersonalitiesMap {
    type Target = BTreeMap<PersonalityId, Personality>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
