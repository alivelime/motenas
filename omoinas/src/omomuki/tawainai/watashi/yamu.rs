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
            if vec!["ある"].contains(&suru.doushita.suru.as_str())
                && if let Some(nani) = &suru.nani {
                    nani.namae().as_str() == "熱"
                } else {
                    false
                }
            {
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
