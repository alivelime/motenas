use crate::hitogata;
use crate::model::Nani;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Shiritai {
    nani: Vec<Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Shitai(shitai) => {
            if shitai.doushita.suru == "知る" && !shitai.hatena {
                return Some(Box::new(Shiritai {
                    nani: shitai.nani.clone(),
                }));
            }
        }
        omomuki::Omomuki::Shite(shite) => {
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
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return match self.nani.len() {
            0 => Result::Message((chara.kaeshi.onegai.shiritai.naniga)()),
            _ => Result::Message((chara.kaeshi.onegai.shiritai.kore)(
                &self.nani[0].donna_namae(),
            )),
        };
    }
}
