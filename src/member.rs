use crate::data::Creature;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub primary_creature: Option<Creature>,
    pub fused_creature: Option<Creature>,
    pub artifact_creature: Option<Creature>,
}
impl Member {
    pub fn get_creature(&self, i: usize) -> &Option<Creature> {
        match i {
            0 => &self.primary_creature,
            1 => &self.fused_creature,
            2 => &self.artifact_creature,
            _ => &None,
        }
    }
    pub fn set_creature(&mut self, i: usize, c: &Option<Creature>) {
        match i {
            0 => self.primary_creature = c.clone(),
            1 => self.fused_creature = c.clone(),
            2 => self.artifact_creature = c.clone(),
            _ => (),
        }
    }
}
