use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ganbaru {}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if vec!["頑張る"].contains(&suru.doushita.suru.as_str()) {
                return Some(Box::new(Ganbaru {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Ganbaru {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::hagemasu::Hagemasu {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
