use crate::service::cotoha;
use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Arigato {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec!["ありがとう", "ありがとうございます"])
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
