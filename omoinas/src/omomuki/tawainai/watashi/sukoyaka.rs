use crate::model::hitogata::Hitogata;
use crate::omomuki::{self, Omomuki, Result, Type};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Sukoyaka {}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match &omomuki.nakami {
        Type::Dearu(dearu) => {
            if dearu.are.has(vec!["元気", "大丈夫"]) && !omomuki.hatena {
                return Some(Box::new(Sukoyaka {}));
            }
        }
        _ => {}
    }
    return None;
}

impl Tumori for Sukoyaka {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::tawainai::yokatta::Yokatta {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
