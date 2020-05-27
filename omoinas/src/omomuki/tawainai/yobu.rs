use crate::model::hitogata::Hitogata;
use crate::omomuki::{Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yobu {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki.yonda {
        return Some(Box::new(Yobu {}));
    }

    return None;
}

impl Tumori for Yobu {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::tawainai::nani::Nani {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
