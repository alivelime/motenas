use crate::cotoha;
use crate::Tumori;

pub struct Hisashiburi {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["久しぶり", "ひさしぶり"]).is_some() {
        return Some(Box::new(Hisashiburi {}));
    }
    return None;
}

impl Tumori for Hisashiburi {
    fn get_kotae(&self) -> String {
        return String::from("よう来たね");
    }
}
