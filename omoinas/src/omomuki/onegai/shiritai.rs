use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Nani;
use crate::omomuki::{Omomuki, Result, Type};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Shiritai {
    nani: Vec<Nani>,
}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match &omomuki.nakami {
        Type::Shitai(shitai) => {
            if shitai.doushita.suru == "知る" && !omomuki.hatena {
                return Some(Box::new(Shiritai {
                    nani: shitai.nani.clone(),
                }));
            }
        }
        Type::Shite(shite) => {
            if shite.doushita.suru == "教える" {
                return Some(Box::new(Shiritai {
                    nani: shite.nani.clone(),
                }));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Shiritai {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return match self.nani.len() {
            0 => Result::Message((chara.kaeshi.onegai.shiritai.naniga)()),
            _ => Result::Message((chara.kaeshi.onegai.shiritai.kore)(
                &self.nani[0].donna_namae(),
            )),
        };
    }
}
