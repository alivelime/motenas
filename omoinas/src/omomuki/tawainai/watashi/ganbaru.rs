use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ganbaru {}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if suru.doushita.suru == "頑張る" {
                return Some(Box::new(Ganbaru {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Ganbaru {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::hagemasu::Hagemasu {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
