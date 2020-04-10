use crate::cotoha;
use crate::hitogata;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Oyasumi {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["おやすみ", "おやすみなさい"]).is_some() {
        return Some(Box::new(Oyasumi {}));
    }

    return None;
}

impl Tumori for Oyasumi {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.aisatsu.hibi.oyasumi)();
    }
}
