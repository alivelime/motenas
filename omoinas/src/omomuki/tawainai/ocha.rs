use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Nani;
use crate::omomuki::{self, Result};
use crate::repository::mono;
use crate::service::cotoha;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ocha {
    yobu: bool,
    nani: Vec<Nani>,
    mono: Nani,
}

pub fn new(ocha: &omomuki::Ocha, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    return Some(Box::new(Ocha {
        yobu: tree
            .has_lemma(vec![
                "おーい",
                "ちょっと",
                "おばあちゃん",
                "ばあちゃん",
                "すみません",
            ])
            .is_some(),
        nani: ocha.nani.clone(),
        mono: ocha.nani[0].clone(),
    }));
}

impl Tumori for Ocha {
    fn kotafu(&self, chara: &Hitogata) -> Box<dyn Tumori> {
        if let Some(n) = self.nani.iter().find(|n| mono::is_mono(&chara.omise, n)) {
            if chara.id.contains(&String::from("/bachan")) {
                return Box::new(Ocha {
                    mono: n.clone(),
                    yobu: self.yobu,
                    nani: self.nani.clone(),
                });
            } else {
                return Box::new(omomuki::toikake::aru::Aru {
                    nani: vec![n.clone()],
                });
            }
        }
        return Box::new(crate::omomuki::tawainai::nani::Nani {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.ocha)(&self.mono.namae(), self.yobu));
    }
}
