use crate::hitogata;
use crate::model::Koto;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Matakuru {
    pub itsu: Option<Koto>,
}

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "来る"
        && omomuki.doushita.toki == omomuki::Toki::Ima
    {
        return Some(Box::new(Matakuru {
            itsu: omomuki.itsu.clone(),
        }));
    }

    return None;
}

impl Tumori for Matakuru {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::iku::matane::Matane {
            itsu: self.itsu.clone(),
        });
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
