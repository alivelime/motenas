use crate::cotoha;
use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::repository::mono;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ocha {
    yobu: bool,
    nani: Vec<omomuki::Nani>,
    mono: omomuki::Nani,
}

pub fn new(ocha: &omomuki::Ocha, tree: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    return Some(Box::new(Ocha {
        yobu: tree
            .has_lemma(vec!["おーい", "ちょっと", "ばあちゃん", "すみません"])
            .is_some(),
        nani: ocha.nani.clone(),
        mono: ocha.nani[0].clone(),
    }));
}

impl Tumori for Ocha {
    fn kotafu(&self, chara: &hitogata::Hitogata) -> Box<dyn Tumori> {
        if let Some(n) = self.nani.iter().find(|n| mono::is_mono(n)) {
            if chara.id == "bachan" {
                return Box::new(omomuki::tawainai::ocha::Ocha {
                    mono: n.clone(),
                    yobu: self.yobu,
                    nani: self.nani.clone(),
                });
            } else {
                return Box::new(omomuki::toikake::aru::Aru {
                    nani: Some(n.clone()),
                });
            }
        }
        return Box::new(crate::omomuki::tawainai::nani::Nani {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.ocha)(&self.mono.namae(), self.yobu));
    }
}
