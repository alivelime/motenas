use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Koto;
use crate::omomuki::Result;
use crate::service::cotoha;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Matane {
    pub itsu: Option<Koto>,
}

pub fn new(tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if tree
        .has_lemma(vec!["また", "ばいばい", "バイバイ", "じゃあ", "さらば"])
        .is_some()
    {
        if let Some(itsu) = tree.get_itsu() {
            return Some(Box::new(Matane { itsu: Some(itsu) }));
        }
    }

    return None;
}

impl Tumori for Matane {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message(if let Some(itsu) = &self.itsu {
            (chara.kaeshi.aisatsu.iku.matane.toki)(itsu.as_str())
        } else {
            (chara.kaeshi.aisatsu.iku.matane.mata)()
        });
    }
}
