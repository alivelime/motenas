use crate::model::hitogata::Hitogata;
use crate::omomuki::{Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Kitanai {
    ng: String,
}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Some(ng) = &omomuki.kitanai {
        return Some(Box::new(Kitanai { ng: ng.clone() }));
    }

    return None;
}

impl Tumori for Kitanai {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.kitanai)(&self.ng));
    }
}
