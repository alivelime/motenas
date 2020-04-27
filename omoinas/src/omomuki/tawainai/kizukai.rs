use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Kizukai {}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Ocha(ocha) => {
            if ocha.hatena && ocha.nani.iter().any(|n| n.has(vec!["元気", "大丈夫"])) {
                return Some(Box::new(Kizukai {}));
            }
        }
        omomuki::Omomuki::Suru(suru) => {
            if suru.hatena
                && suru.doushita.suru == "ひく"
                && match &suru.nani {
                    Some(nani) => nani.has(vec!["風邪"]),
                    None => false,
                }
            {
                return Some(Box::new(Kizukai {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Kizukai {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::bochibochi::Bochibochi {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.kizukai)());
    }
}
