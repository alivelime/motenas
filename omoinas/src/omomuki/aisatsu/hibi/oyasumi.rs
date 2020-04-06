use crate::cotoha;
use crate::Tumori;

pub struct Oyasumi {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["おやすみ", "おやすみなさい"]).is_some() {
        return Some(Box::new(Oyasumi {}));
    }

    return None;
}
impl Tumori for Oyasumi {
    fn get_kotae(&self) -> String {
        return String::from("はい、おやすみ");
    }
}
