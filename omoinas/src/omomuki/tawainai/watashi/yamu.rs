use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Omomuki, Result, Type};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yamu {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match &omomuki.nakami {
        Type::Keiyou(keiyou) => {
            if keiyou.dou == "痛い" {
                return Some(Box::new(Yamu {}));
            }
        }
        Type::Suru(suru) => {
            if suru.doushita.suru == "ある" && suru.nani.iter().any(|n| n.has(vec!["熱"])) {
                return Some(Box::new(Yamu {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Yamu {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::kizukai::Kizukai {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
