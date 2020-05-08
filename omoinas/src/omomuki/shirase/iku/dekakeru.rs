use crate::hitogata;
use crate::model::{Koto, Nani};
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Dekakeru {
    pub itsu: Option<Koto>,
    pub doko: Option<Koto>,
    pub nani: Vec<Nani>,
}

pub fn new(omomuki: &omomuki::Suru) -> Option<Box<dyn Tumori>> {
    // TODO して来る
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "行く"
        && omomuki.doushita.toki == omomuki::Toki::Ima
    {
        return Some(Box::new(Dekakeru {
            itsu: omomuki.itsu.clone(),
            doko: omomuki.doko.clone(),
            nani: omomuki
                .nani
                .iter()
                .map(|n| {
                    if n.mono.iter().any(|m| m == "お昼") {
                        Nani {
                            donna: None,
                            mono: vec![Koto::from_str("お昼"), Koto::from_str("食べ")],
                        }
                    } else {
                        n.clone()
                    }
                })
                .collect(),
        }));
    }

    return None;
}

impl Tumori for Dekakeru {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::aisatsu::iku::ittera::Ittera {
            itsu: self.itsu.clone(),
            doko: self.doko.clone(),
            nani: self.nani.clone(),
        });
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
