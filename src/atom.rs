use fermi::Atom;

use crate::state::UrlState;
use crate::url_save::UrlSave;

pub const URL_STATE: Atom<UrlState> = Atom(|_| {
    UrlSave::get_from_url()
        .map(|s| s.to_state())
        .unwrap_or(UrlState::default())
});
