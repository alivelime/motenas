use crate::cotoha;
use crate::Tumori;

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["ただいま"]).is_some() {
        return Some(Box::new(crate::omomuki::aisatsu::kuru::okaeri::Okaeri {}));
    }
    return None;
}
