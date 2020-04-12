use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Yoku {}

pub fn new(omomuki: &omomuki::Suru, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "来る"
        && omomuki.doushita.toki == omomuki::Toki::Mukashi
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
