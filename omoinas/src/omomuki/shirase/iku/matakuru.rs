use crate::model::hitogata::Hitogata;
use crate::model::kotoba::{Koto, Kotoba};
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Matakuru {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
}

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == Kotoba::from_vec(vec!["来る", "くる"])
        && omomuki.doushita.toki == omomuki::Toki::Ima
    {
        return Some(Box::new(Matakuru {
            itsu: omomuki.itsu.clone(),
            doko: omomuki.doko.clone(),
        }));
    }

    return None;
}

impl Tumori for Matakuru {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::iku::matane::Matane {
            itsu: self.itsu.clone(),
            doko: self.doko.clone(),
        });
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
