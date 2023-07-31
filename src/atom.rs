use crate::embed_data;
use data::Data;
use fermi::Atom;

pub static DATA: Atom<Data> = Atom(|_| embed_data::load());
