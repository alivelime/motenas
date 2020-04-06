use crate::cotoha;
use crate::Tumori;

pub struct Hajimemashite {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["はじめまして", "初めまして"]).is_some() {
        return Some(Box::new(Hajimemashite {}));
    }
    return None;
}

impl Tumori for Hajimemashite {
    fn get_kotae(&self) -> String {
        return String::from("はい、ご丁寧にどうも");
    }
}
