use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Neru {}

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "寝る"
        && omomuki.doushita.toki == omomuki::Toki::Ima
    {
        return Some(Box::new(Neru {}));
    } else {
        return None;
    }
}

impl Tumori for Neru {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::hibi::oyasumi::Oyasumi {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
