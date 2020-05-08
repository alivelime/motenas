use crate::cotoha;
use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Douitashimashite {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["どう致しまして"]).is_some() {
        return Some(Box::new(Douitashimashite {}));
    }
    return None;
}

impl Tumori for Douitashimashite {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.douitashimashite)());
    }
}
