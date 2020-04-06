use crate::cotoha;
use crate::kaeshi;
use crate::Tumori;

pub struct Ohayo {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec!["おはよう", "おはようございます"])
        .is_some()
    {
        return Some(Box::new(Ohayo {}));
    }
    return None;
}

impl Tumori for Ohayo {
    fn get_kotae(&self) -> String {
        return String::from(kaeshi::CHAR.aisatsu.hibi.ohayo);
    }
}
