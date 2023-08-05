use data::r#trait::Trait;

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
}
