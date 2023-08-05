use data::r#trait::Trait;
use data::stat::Stat;

#[derive(Clone, Debug, PartialEq)]
pub struct UrlState<'a> {
    pub party: [Member<'a>; 6],
}

impl Default for UrlState<'_> {
    fn default() -> Self {
        UrlState {
            party: [
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
                Member::new(),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Member<'a> {
    pub traits: [Option<&'a Trait>; 3],
}

impl<'a> Member<'a> {
    pub fn new() -> Self {
        Member {
            traits: [None, None, None],
        }
    }

    pub fn sprite(&self) -> Option<String> {
        self.traits[0].and_then(|t| t.sprite.clone())
    }

    pub fn class(&self) -> Option<String> {
        self.traits[1].map(|t| t.class.clone())
    }

    pub fn creature(&self) -> Option<String> {
        self.traits[0].map(|t| t.creature.clone())
    }

    pub fn family(&self) -> Option<String> {
        self.traits[0].map(|t| t.family.clone())
    }

    pub fn stats(&self, stat: Stat) -> Option<u8> {
        flatten_stat(
            self.traits[0].and_then(|t| t.stat(stat.clone())),
            self.traits[1].and_then(|t| t.stat(stat.clone())),
        )
    }
}

fn flatten_stat(a: Option<u8>, b: Option<u8>) -> Option<u8> {
    match (a, b) {
        (Some(x), Some(y)) => Some(((x + y) as f64 / 2f64) as u8),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}
