use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yamu {}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Keiyou(keiyou) => {
            if keiyou.dou == "痛い" {
                return Some(Box::new(Yamu {}));
            }
        }
        omomuki::Omomuki::Suru(suru) => {
            if suru.doushita.suru == "ある" && suru.nani.iter().any(|n| n.has(vec!["熱"])) {
                return Some(Box::new(Yamu {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Yamu {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::kizukai::Kizukai {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
