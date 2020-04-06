use crate::cotoha;
use crate::Tumori;

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
    fn get_kotae(&self) -> String {
        return format!("{}とはなんだい!\nもっと綺麗な言葉をお使い!", self.ng);
    }
}
