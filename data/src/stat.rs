use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
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
