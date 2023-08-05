use fermi::Atom;

use crate::state::UrlState;
use crate::url_save;

pub const URL_STATE: Atom<UrlState> =
    Atom(|_| url_save::get_from_url().unwrap_or(UrlState::default()));

pub const SHOW_TRAITS: Atom<bool> = Atom(|_| true);
pub const SHOW_SPELLS: Atom<bool> = Atom(|_| true);
