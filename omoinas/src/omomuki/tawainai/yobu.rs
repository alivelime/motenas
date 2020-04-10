use crate::cotoha;
use crate::hitogata;
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
        ])
        .is_some()
    {
        return Some(Box::new(Yobu {}));
    }

    return None;
}

impl Tumori for Yobu {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::tawainai::nani::Nani {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
