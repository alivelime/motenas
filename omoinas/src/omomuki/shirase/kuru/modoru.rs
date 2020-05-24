use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Kotoba;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Modoru {}

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    // 行ってきた
    if omomuki.dare == None
        && omomuki.doushita.suru == Kotoba::from_vec(vec!["戻る", "帰る"])
        && omomuki.doushita.toki == omomuki::Toki::Mukashi
    {
        return Some(Box::new(Modoru {}));
    }

    return None;
}

impl Tumori for Modoru {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::kuru::okaeri::Okaeri {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
