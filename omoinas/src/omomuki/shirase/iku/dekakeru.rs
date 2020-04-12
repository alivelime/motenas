use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Dekakeru {
    pub itsu: Option<String>,
    pub doko: Option<String>,
    pub nani: Option<omomuki::Nani>,
}

pub fn new(omomuki: &omomuki::Suru, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    // TODO して来る
    if omomuki.dare.is_none()
        && omomuki.doushita.suru == "行く"
        && omomuki.doushita.toki == omomuki::Toki::Ima
    {
        let mut n = omomuki.nani.clone();
        if if let Some(nani) = &omomuki.nani {
            nani.mono == "お昼"
        } else {
            false
        } {
            n = Some(omomuki::Nani {
                mono: String::from("お昼ご飯食べ"),
                donna: None,
            });
        }
        return Some(Box::new(Dekakeru {
            itsu: None,
            doko: omomuki.doko.clone(),
            nani: n,
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
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
