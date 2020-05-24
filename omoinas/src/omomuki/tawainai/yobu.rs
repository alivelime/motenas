use crate::service::cotoha;
use crate::model::hitogata::Hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yobu {}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec![
            "おーい",
            "ばあちゃん",
            "ばあさん",
            "やっほー",
            "やあ",
            "ねえねえ",
            "ねえ",
            "すみません",
        ])
        .is_some()
    {
        return Some(Box::new(Yobu {}));
    }

    return None;
}

impl Tumori for Yobu {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::tawainai::nani::Nani {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
