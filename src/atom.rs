use fermi::Atom;

use data::Data;

use crate::embed_data;

pub static DATA: Atom<Data> = Atom(|_| embed_data::load());
