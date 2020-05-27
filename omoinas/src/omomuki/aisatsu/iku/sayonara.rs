use crate::model::hitogata::Hitogata;
use crate::omomuki::{Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Sayonara {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki.has_hitokoto(vec!["さようなら"]).is_some() {
        return Some(Box::new(Sayonara {}));
    }

    return None;
}

impl Tumori for Sayonara {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.aisatsu.iku.sayonara)());
    }
}
