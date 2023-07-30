use implicit_clone::unsync::IString;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stat {
    Health,
    Attack,
    Intelligence,
    Defense,
    Speed,
}

impl Display for Stat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Stat::Health => f.write_str("health"),
            Stat::Attack => f.write_str("attack"),
            Stat::Intelligence => f.write_str("intelligence"),
            Stat::Defense => f.write_str("defense"),
            Stat::Speed => f.write_str("speed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Personality {
    pub id: i8,
    pub name: IString,
    pub positive: Stat,
    pub negative: Stat,
}

impl Personality {
    fn new(id: i8, name: &'static str, positive: Stat, negative: Stat) -> Personality {
        Self {
            id,
            name: IString::from(name),
            positive,
            negative,
        }
    }

    pub fn load() -> Vec<Personality> {
        let xs: Vec<(&'static str, Stat, Stat)> = vec![("Analytical", Stat::Intelligence, Stat::Health), ("Apathetic", Stat::Health, Stat::Defense), ("Bashful", Stat::Speed, Stat::Attack), ("Brave", Stat::Attack, Stat::Defense), ("Brutal", Stat::Attack, Stat::Intelligence), ("Careful", Stat::Intelligence, Stat::Attack), ("Clever", Stat::Intelligence, Stat::Defense), ("Daring", Stat::Attack, Stat::Speed), ("Gentle", Stat::Defense, Stat::Intelligence), ("Indifferent", Stat::Health, Stat::Intelligence), ("Lazy", Stat::Health, Stat::Attack), ("Nervous", Stat::Speed, Stat::Health), ("Peaceful", Stat::Defense, Stat::Speed), ("Protective", Stat::Defense, Stat::Attack), ("Reckless", Stat::Attack, Stat::Health), ("Relaxed", Stat::Health, Stat::Speed), ("Selfless", Stat::Defense, Stat::Health), ("Shrewd", Stat::Intelligence, Stat::Speed), ("Shy", Stat::Speed, Stat::Intelligence), ("Timid", Stat::Speed, Stat::Defense)];
        xs.iter()
            .enumerate()
            .map(|(i, x)| Personality::new((i + 1) as i8, x.0, x.1.clone(), x.2.clone()))
            .collect()
    }

    pub fn get_by_id(xs: &Vec<Personality>, id: i8) -> Option<Personality> {
        xs.iter().find(|x| x.id == id).map(|x| x.clone())
    }

    pub fn get_by_stat(
        xs: &Vec<Personality>,
        positive: &Option<Stat>,
        negative: &Option<Stat>,
    ) -> Option<Personality> {
        match (positive, negative) {
            (Some(p), Some(n)) => xs
                .iter()
                .find(|x| x.positive == *p && x.negative == *n)
                .map(|x| x.clone()),
            _ => None,
        }
    }
}
