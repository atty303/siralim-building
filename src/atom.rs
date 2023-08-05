use fermi::Atom;

use crate::state::UrlState;
use crate::url_save;

pub const URL_STATE: Atom<UrlState> =
    Atom(|_| url_save::get_from_url().unwrap_or(UrlState::default()));
