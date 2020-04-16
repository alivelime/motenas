use crate::cotoha;
use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Dekakeru {
    pub itsu: Option<String>,
    pub doko: Option<String>,
    pub nani: Option<String>,
}

pub fn new(omomuki: &omomuki::Suru, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    // TODO して来る
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "行く"
        && omomuki.doushita.toki == omomuki::Toki::Ima
    {
        return Some(Box::new(Dekakeru {
            itsu: None,
            doko: omomuki.doko.clone(),
            nani: if let Some(nani) = &omomuki.nani {
                if nani.mono.contains(&String::from("お昼")) {
                    Some(String::from("お昼食べに"))
                } else {
                    Some(nani.mono.join(""))
                }
            } else {
                None
            },
        }));
    }

    return None;
}

impl Tumori for Dekakeru {
    fn kotafu(&self) -> Box<dyn Tumori> {
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
