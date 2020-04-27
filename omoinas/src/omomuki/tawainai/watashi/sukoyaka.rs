use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Sukoyaka {}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Dearu(dearu) => {
            if (dearu.kore.namae().as_str() == "私" && dearu.are.namae() == "元気")
                || (dearu.kore.namae().as_str() == "私" && dearu.are.namae() == "大丈夫")
            {
                return Some(Box::new(Sukoyaka {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Sukoyaka {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::yokatta::Yokatta {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
