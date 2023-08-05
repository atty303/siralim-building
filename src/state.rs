use data::r#trait::Trait;

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub traits: Vec<Option<Trait>>,
}

impl Member {
    pub fn new() -> Self {
        Member {
            traits: vec![None, None, None],
        }
    }
}
