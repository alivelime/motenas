use crate::cotoha;
use crate::Tumori;

pub struct Sayonara {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["さようなら", "さよなら"]).is_some() {
        return Some(Box::new(Sayonara {}));
    }

    return None;
}

impl Tumori for Sayonara {
    fn get_kotae(&self) -> String {
        return String::from("はい、さようなら");
    }
}
