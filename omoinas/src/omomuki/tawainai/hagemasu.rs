use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Omomuki, Result, Type};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Hagemasu {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match &omomuki.nakami {
        Type::Shite(shite) => {
            if shite.doushita.suru == "頑張る" {
                return Some(Box::new(Hagemasu {}));
            }
        }
        Type::Suru(suru) => {
            if suru.doushita.suru == "する" && suru.nani.iter().any(|n| n.has(vec!["応援"])) {
                return Some(Box::new(Hagemasu {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Hagemasu {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::arigato::Arigato {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.hagemasu)());
    }
}
