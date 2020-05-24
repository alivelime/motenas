use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Shite {}

pub fn new(_shite: &omomuki::Suru) -> Box<dyn Tumori> {
    return Box::new(Shite {});
}

impl Tumori for Shite {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.onegai.shite)());
    }
}
