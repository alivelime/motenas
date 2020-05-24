use crate::service::cotoha;
use crate::model::hitogata::Hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Tadaima {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree.has_lemma(vec!["ただいま"]).is_some() {
        return Some(Box::new(Tadaima {}));
    }
    return None;
}

impl Tumori for Tadaima {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::kuru::okaeri::Okaeri {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
