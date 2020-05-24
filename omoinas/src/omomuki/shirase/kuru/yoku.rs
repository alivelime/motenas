use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yoku {}

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "来る"
        && omomuki.doushita.toki == omomuki::Toki::Mukashi
    {
        return Some(Box::new(Yoku {}));
    }
    return None;
}

impl Tumori for Yoku {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::aisatsu::kuru::irasshai::Irasshai {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
