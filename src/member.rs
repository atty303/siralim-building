use data::r#trait::Trait;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub primary_trait: Option<Trait>,
    pub fused_trait: Option<Trait>,
    pub artifact_trait: Option<Trait>,
}
impl Member {
    pub fn get_creature(&self, i: usize) -> &Option<Trait> {
        match i {
            0 => &self.primary_trait,
            1 => &self.fused_trait,
            2 => &self.artifact_trait,
            _ => &None,
        }
    }
    pub fn set_creature(&mut self, i: usize, c: &Option<Trait>) {
        match i {
            0 => self.primary_trait = c.clone(),
            1 => self.fused_trait = c.clone(),
            2 => self.artifact_trait = c.clone(),
            _ => (),
        }
    }
}
