use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Monku {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki
        .has(vec!["えー", "うそ", "嘘つき", "違う", "ない", "笑い", "嘘"])
        .is_some()
        || (omomuki.has(vec!["あれ"]).is_some() && omomuki.hatena)
    {
        return Some(Box::new(Monku {}));
    }

    return None;
}

impl Tumori for Monku {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::ayamaru::Ayamaru {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
