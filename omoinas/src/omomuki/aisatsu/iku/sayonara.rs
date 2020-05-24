use crate::service::cotoha;
use crate::model::hitogata::Hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Sayonara {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["さようなら", "さよなら"]).is_some() {
        return Some(Box::new(Sayonara {}));
    }

    return None;
}

impl Tumori for Sayonara {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.aisatsu.iku.sayonara)());
    }
}
