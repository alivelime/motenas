use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Matakuru {
    pub itsu: Option<String>,
}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if omomuki.dare.is_none() {
        if if let Some(d) = &omomuki.doushita {
            d.suru == "来る" && d.toki == omomuki::Toki::Ima
        } else {
            false
        } {
            return Some(Box::new(Matakuru {
                itsu: omomuki.itsu.clone(),
            }));
        }
    }

    return None;
}

impl Tumori for Matakuru {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(crate::omomuki::aisatsu::iku::matane::Matane {
            itsu: self.itsu.clone(),
        });
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
