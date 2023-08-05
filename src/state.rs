use data::r#trait::Trait;

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
