use apache_avro::AvroSchema;
use implicit_clone::unsync::IString;
use serde::{Deserialize, Serialize};
use tantivy::aggregation::Key;

#[derive(Debug, Clone)]
pub struct Keyword {
    pub name: IString,
    pub category: IString,
    pub icon: IString,
}

impl Keyword {
    fn new(name: &str, category: &str, icon: &str) -> Keyword {
        Self {
            name: IString::from(String::from(name)),
            category: IString::from(String::from(category)),
            icon: IString::from(String::from(icon)),
        }
    }

    pub fn load() -> Vec<Keyword> {
        vec![
            Keyword::new("Timeline", "timeline", "battle_history_0.png"),
            Keyword::new("Attack", "stat", "attack.png"),
            Keyword::new("Intelligence", "stat", "intelligence.png"),
            Keyword::new("Health", "stat", "health.png"),
            Keyword::new("Defense", "stat", "defense.png"),
            Keyword::new("Speed", "stat", "speed.png"),
            Keyword::new("Charge", "stat", "library_spell_gems_0.png"),
            Keyword::new("Charges", "stat", "library_spell_gems_0.png"),
            Keyword::new("Attacks", "action", "battle_attack_0.png"),
            Keyword::new("Attacked", "action", "battle_attack_0.png"),
            Keyword::new("Attacking", "action", "battle_attack_0.png"),
            Keyword::new("Cast", "action", "battle_cast_0.png"),
            Keyword::new("Casts", "action", "battle_cast_0.png"),
            Keyword::new("Defend", "action", "battle_defend_0.png"),
            Keyword::new("Defends", "action", "battle_defend_0.png"),
            Keyword::new("Defending", "action", "battle_defend_0.png"),
            Keyword::new("Provoke", "action", "battle_provoke_0.png"),
            Keyword::new("Provokes", "action", "battle_provoke_0.png"),
            Keyword::new("Provoked", "action", "battle_provoke_0.png"),
            Keyword::new("Provoking", "action", "battle_provoke_0.png"),
        ]
    }
}
