use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Shitai {}

pub fn new(_shitai: &omomuki::Suru) -> Box<dyn Tumori> {
    return Box::new(Shitai {});
}

impl Tumori for Shitai {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.onegai.shitai)());
    }
}
