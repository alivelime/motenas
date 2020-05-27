use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Nani;
use crate::omomuki::{self, Omomuki, Result, Type};
use crate::repository::mono;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Ocha {
    yobareta: bool,
    nani: Vec<Nani>,
    mono: Nani,
}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    if let Type::Ocha(ocha) = &omomuki.nakami {
        return Some(Box::new(Ocha {
            yobareta: omomuki.yonda,
            nani: ocha.nani.clone(),
            mono: ocha.nani[0].clone(),
        }));
    }
    return None;
}

impl Tumori for Ocha {
    fn kotafu(&self, chara: &Hitogata) -> Box<dyn Tumori> {
        if let Some(n) = self.nani.iter().find(|n| mono::is_mono(&chara.omise, n)) {
            if chara.id.contains(&String::from("/bachan")) {
                return Box::new(Ocha {
                    mono: n.clone(),
                    yobareta: self.yobareta,
                    nani: self.nani.clone(),
                });
            } else {
                return Box::new(omomuki::toikake::aru::Aru {
                    nani: vec![n.clone()],
                });
            }
        }
        return Box::new(crate::omomuki::tawainai::nani::Nani {});
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.tawainai.ocha)(
            &self.mono.namae(),
            self.yobareta,
        ));
    }
}
