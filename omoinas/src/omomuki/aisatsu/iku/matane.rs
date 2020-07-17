use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Koto;
use crate::omomuki::{Omomuki, Result, Type};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Matane {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Type::Tawainai(tawainai) = &omomuki.nakami {
        if omomuki
            .has(vec![
                "また",
                "またね",
                "ばいばい",
                "バイバイ",
                "じゃあ",
                "さらば",
            ])
            .is_some()
        {
            return Some(Box::new(Matane {
                itsu: tawainai.itsu.clone(),
                doko: tawainai.doko.clone(),
            }));
        }
    }

    return None;
}

impl Tumori for Matane {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message(if let Some(itsu) = &self.itsu {
            (chara.kaeshi.aisatsu.iku.matane.toki)(itsu.as_str())
        } else if let Some(doko) = &self.doko {
            (chara.kaeshi.aisatsu.iku.matane.tokoro)(doko.as_str())
        } else {
            (chara.kaeshi.aisatsu.iku.matane.mata)()
        });
    }
}
