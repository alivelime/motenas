use crate::cotoha;
use crate::kaeshi;
use crate::Tumori;

pub struct Konnichiwa {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["こんにちは"]).is_some() {
        return Some(Box::new(Konnichiwa {}));
    }
    return None;
}
impl Tumori for Konnichiwa {
    fn get_kotae(&self) -> String {
        return String::from(kaeshi::CHAR.aisatsu.hibi.konnichiwa);
    }
}
