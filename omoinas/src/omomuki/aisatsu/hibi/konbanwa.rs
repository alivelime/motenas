use crate::cotoha;
use crate::Tumori;

pub struct Konbanwa {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["こんばんは"]).is_some() {
        return Some(Box::new(Konbanwa {}));
    }
    return None;
}
impl Tumori for Konbanwa {
    fn get_kotae(&self) -> String {
        return String::from("はい、こんばんは");
    }
}
