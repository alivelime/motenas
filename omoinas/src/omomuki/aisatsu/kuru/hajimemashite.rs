use crate::cotoha;
use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Hajimemashite {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["はじめまして", "初めまして"]).is_some() {
        return Some(Box::new(Hajimemashite {}));
    }
    return None;
}

impl Tumori for Hajimemashite {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.aisatsu.kuru.hajimemashite)());
    }
}
