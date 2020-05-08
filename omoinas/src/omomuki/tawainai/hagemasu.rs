use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Hagemasu {}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Shite(shite) => {
            if shite.doushita.suru == "頑張る" {
                return Some(Box::new(Hagemasu {}));
            }
        }
        omomuki::Omomuki::Suru(suru) => {
            if suru.doushita.suru == "する" && suru.nani.iter().any(|n| n.has(vec!["応援"])) {
                return Some(Box::new(Hagemasu {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Hagemasu {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::arigato::Arigato {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.hagemasu)());
    }
}
