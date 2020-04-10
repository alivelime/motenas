use crate::cotoha;
use crate::hitogata;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Kitanai {
    ng: String,
}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(ng) = tree.has_lemma(vec!["おい", "ババア", "ばばあ", "くそ", "死ね"]) {
        return Some(Box::new(Kitanai { ng: ng }));
    }

    return None;
}

impl Tumori for Kitanai {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.kitanai)(&self.ng);
    }
}
