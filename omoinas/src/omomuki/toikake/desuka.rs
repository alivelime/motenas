use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Nani;
use crate::model::mono::Desu;
use crate::omomuki::{Omomuki, Result, Type};
use crate::repository::mono;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Desuka {
    pub kore: Option<Nani>,
    pub are: Nani,
}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match &omomuki.nakami {
        Type::Dearu(dearu) => {
            if omomuki.hatena {
                return Some(Box::new(Desuka {
                    kore: Some(dearu.kore.clone()),
                    are: dearu.are.clone(),
                }));
            }
        }
        Type::Keiyou(keiyou) => {
            if omomuki.hatena {
                return Some(Box::new(Desuka {
                    kore: keiyou.nani.last().cloned(),
                    are: Nani {
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
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return if let Some(kore) = &self.kore {
            match mono::desuka(&chara.omise, &kore, &self.are) {
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
                Desu::Mono(mono, category) => match mono.len() {
                    1..=7 => Result::Mono(
                        (chara.kaeshi.toikake.desuka.iroiro)(
                            mono.iter()
                                .map(|m| m.category.last().unwrap().to_string())
                                .collect(),
                        ),
                        mono,
                    ),
                    _ => Result::Message((chara.kaeshi.toikake.desuka.iroiro)(category)),
                },
                Desu::Ikura(mono) => match mono.len() {
                    1..=7 => Result::Mono(
                        (chara.kaeshi.toikake.desuka.ikura)(
                            mono.iter()
                                .map(|m| (m.category.last().unwrap().as_str(), m.neuchi))
                                .collect(),
                        ),
                        mono,
                    ),
                    _ => Result::Message((chara.kaeshi.toikake.desuka.ikura)(
                        mono.iter()
                            .map(|m| (m.category.last().unwrap().as_str(), m.neuchi))
                            .collect(),
                    )),
                },
            }
        } else {
            Result::Message((chara.kaeshi.toikake.desuka.naniga)())
        };
    }
}
