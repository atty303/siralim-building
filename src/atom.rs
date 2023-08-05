use fermi::Atom;

use crate::state::Member;

pub const PARTY: Atom<Vec<Member>> = Atom(|_| vec![Member::new(), Member::new()]);
