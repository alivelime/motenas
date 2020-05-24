use crate::service::cotoha;
use crate::model::hitogata::Hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Kitanai {
    ng: String,
}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if let Some(ng) = tree.has_lemma(vec![
        "おい",
        "ババア",
        "ばばあ",
        "くそ",
        "死ね",
        "馬鹿",
        "ふざける",
        "おっぱい",
        "パンツ",
    ]) {
        return Some(Box::new(Kitanai { ng: ng }));
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
