use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Arigato {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if omomuki
        .has_hitokoto(vec!["ありがとう", "ありがとうございます"])
        .is_some()
    {
        return Some(Box::new(Arigato {}));
    }
    return None;
}

impl Tumori for Arigato {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::douitashimashite::Douitashimashite {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.arigato)());
    }
}
