use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Keyword {
    pub name: &'static str,
    pub category: &'static str,
    pub icon: &'static str,
}

impl Keyword {
    fn new(name: &'static str, category: &'static str, icon: &'static str) -> Keyword {
        Self {
            name,
            category,
            icon,
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

pub struct KeywordsMap {
    inner: HashMap<String, Keyword>,
}

impl KeywordsMap {
    pub fn new() -> KeywordsMap {
        let inner = Keyword::load()
            .into_iter()
            .map(|e| (e.name.to_string(), e))
            .collect::<HashMap<_, _>>();
        Self { inner }
    }
}

impl Deref for KeywordsMap {
    type Target = HashMap<String, Keyword>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
