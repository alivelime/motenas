use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Dearu {}

pub fn new(_dearu: &omomuki::Dearu) -> Box<dyn Tumori> {
    return Box::new(Dearu {});
}

impl Tumori for Dearu {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::sounanda::Sounanda {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.onegai.shitai)());
    }
}
