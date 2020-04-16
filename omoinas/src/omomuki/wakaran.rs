use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Wakaran {}

pub fn new() -> Box<dyn Tumori> {
    return Box::new(Wakaran {});
}

impl Tumori for Wakaran {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.wakaran.wakaran)());
    }
}
