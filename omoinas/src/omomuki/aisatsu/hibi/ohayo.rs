use crate::cotoha;
use crate::hitogata;
use crate::Tumori;

#[derive(Clone, Debug)]
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
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.aisatsu.hibi.ohayo)();
    }
}
