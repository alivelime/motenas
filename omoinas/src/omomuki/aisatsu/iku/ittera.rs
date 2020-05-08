use crate::hitogata;
use crate::model::{Koto, Nani};
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ittera {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
    pub nani: Vec<Nani>,
}

impl Tumori for Ittera {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message(if let Some(doko) = &self.doko {
            if let Some(itsu) = &self.itsu {
                (chara.kaeshi.aisatsu.iku.ittera.tokitokoro)(itsu.as_str(), doko.as_str())
            } else {
                (chara.kaeshi.aisatsu.iku.ittera.tokoro)(doko.as_str())
            }
        } else if let Some(itsu) = &self.itsu {
            (chara.kaeshi.aisatsu.iku.ittera.toki)(itsu.as_str())
        } else if self.nani.len() > 0 {
            (chara.kaeshi.aisatsu.iku.ittera.tokoro)(self.nani[0].namae().as_str())
        } else {
            (chara.kaeshi.aisatsu.iku.ittera.ittera)()
        });
    }
}
