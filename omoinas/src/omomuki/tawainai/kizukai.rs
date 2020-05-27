use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Omomuki, Result, Type};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Kizukai {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match &omomuki.nakami {
        Type::Ocha(ocha) => {
            if omomuki.hatena && ocha.nani.iter().any(|n| n.has(vec!["元気", "大丈夫"])) {
                return Some(Box::new(Kizukai {}));
            }
        }
        Type::Suru(suru) => {
            if omomuki.hatena
                && suru.doushita.suru == "ひく"
                && suru.nani.iter().any(|n| n.has(vec!["風邪"]))
            {
                return Some(Box::new(Kizukai {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Kizukai {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::bochibochi::Bochibochi {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.kizukai)());
    }
}
