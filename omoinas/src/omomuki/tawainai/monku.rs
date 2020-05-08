use crate::cotoha;
use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Monku {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec!["えー", "うそ", "嘘つき", "違う", "ない", "笑い", "嘘"])
        .is_some()
        || (tree.has_lemma(vec!["あれ"]).is_some() && tree.is_hatena())
    {
        return Some(Box::new(Monku {}));
    }

    return None;
}

impl Tumori for Monku {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::ayamaru::Ayamaru {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
