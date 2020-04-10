use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yoku {}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && if let Some(d) = &omomuki.doushita {
            d.suru == "来る" && d.toki == omomuki::Toki::Mukashi
        } else {
            false
        }
    {
        return Some(Box::new(Yoku {}));
    }
    return None;
}

impl Tumori for Yoku {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(omomuki::aisatsu::kuru::irasshai::Irasshai {});
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
