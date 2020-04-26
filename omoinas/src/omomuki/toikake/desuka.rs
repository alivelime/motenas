use crate::hitogata;
use crate::model;
use crate::model::mono::Desu;
use crate::omomuki::{self, Result};
use crate::repository::mono;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Desuka {
    pub kore: Option<model::Nani>,
    pub are: model::Nani,
}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Dearu(dearu) => {
            if dearu.hatena {
                return Some(Box::new(Desuka {
                    kore: Some(dearu.kore.clone()),
                    are: dearu.are.clone(),
                }));
            }
        }
        omomuki::Omomuki::Keiyou(keiyou) => {
            if keiyou.hatena {
                return Some(Box::new(Desuka {
                    kore: keiyou.nani.clone(),
                    are: model::Nani {
                        donna: Some(keiyou.dou.clone()),
                        mono: Vec::new(),
                    },
                }));
            }
        }
        _ => {}
    };
    return None;
}

impl Tumori for Desuka {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return if let Some(kore) = &self.kore {
            match mono::desuka(&kore, &self.are) {
                Desu::Wakaran(nai) => Result::Message((chara.kaeshi.toikake.desuka.wakaran)(&nai)),
                Desu::Subete() => Result::Message((chara.kaeshi.toikake.desuka.subete)()),
                Desu::Doreka() => Result::Message((chara.kaeshi.toikake.desuka.doreka)()),
                Desu::Chigau() => Result::Message((chara.kaeshi.toikake.desuka.chigau)()),
                Desu::IsCategory(category) => {
                    Result::Message((chara.kaeshi.toikake.desuka.dayo)(category.as_str()))
                }
                Desu::Category(category) => {
                    Result::Message((chara.kaeshi.toikake.desuka.iroiro)(category))
                }
            }
        } else {
            Result::Message((chara.kaeshi.toikake.desuka.naniga)())
        };
    }
}
