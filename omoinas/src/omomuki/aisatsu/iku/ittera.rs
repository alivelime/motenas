use crate::hitogata;
use crate::omomuki::Result;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ittera {
    pub itsu: Option<String>,
    pub doko: Option<String>,
    pub nani: Option<String>,
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
        } else if let Some(nani) = &self.nani {
            (chara.kaeshi.aisatsu.iku.ittera.tokoro)(nani.as_str())
        } else {
            (chara.kaeshi.aisatsu.iku.ittera.ittera)()
        });
    }
}
