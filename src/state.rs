use data::r#trait::Trait;

#[derive(Clone, Debug, PartialEq)]
pub struct UrlState<'a> {
    pub party: Vec<Member<'a>>,
}

impl Default for UrlState<'_> {
    fn default() -> Self {
        UrlState {
            party: vec![
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
    pub traits: Vec<Option<&'a Trait>>,
}

impl<'a> Member<'a> {
    pub fn new() -> Self {
        Member {
            traits: vec![None, None, None],
        }
    }
}
