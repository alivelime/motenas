use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Neru {}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && if let Some(d) = &omomuki.doushita {
            d.suru == "寝る" && d.toki == omomuki::Toki::Ima
        } else {
            false
        }
    {
        return Some(Box::new(Neru {}));
    }
    return None;
}

impl Tumori for Neru {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::hibi::oyasumi::Oyasumi {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
