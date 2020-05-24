use crate::model::hitogata::Hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Wakaran {}

pub fn new() -> Box<dyn Tumori> {
    return Box::new(Wakaran {});
}

impl Tumori for Wakaran {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.wakaran.wakaran)());
    }
}
