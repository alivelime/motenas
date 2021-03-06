use crate::model::hitogata::Hitogata;
use crate::omomuki::{Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Hisashiburi {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki
        .has_hitokoto(vec!["久しぶり", "ひさしぶり"])
        .is_some()
    {
        return Some(Box::new(Hisashiburi {}));
    }
    return None;
}

impl Tumori for Hisashiburi {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.aisatsu.kuru.hisashiburi)());
    }
}
