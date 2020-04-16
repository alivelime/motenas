use crate::cotoha;
use crate::hitogata;
use crate::omomuki::{self, Result};
use crate::repository::mono;
use crate::Tumori;

use crate::model::mono::MonoResult;

#[derive(Clone, Debug)]
pub struct Aru {
    pub nani: Option<omomuki::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    return match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if vec!["ある"].contains(&suru.doushita.suru.as_str()) {
                Some(Box::new(Aru {
                    nani: suru.nani.clone(),
                }))
            } else {
                None
            }
        }
        omomuki::Omomuki::Keiyou(keiyou) => {
            if keiyou.dou == "ない" {
                Some(Box::new(Aru {
                    nani: keiyou.nani.clone(),
                }))
            } else {
                None
            }
        }
        _ => None,
    };
}

impl Tumori for Aru {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return match mono::get_mono(&self.nani) {
            MonoResult::Category(category) => {
                Result::Message((chara.kaeshi.toikake.aru.iroiro)(category))
            }
            MonoResult::Mono(mono) => Result::Mono(
                if mono.len() == 1 {
                    (chara.kaeshi.toikake.aru.aru)(mono[0].category.last().unwrap())
                } else {
                    (chara.kaeshi.toikake.aru.iroiro)(
                        mono.iter()
                            .map(|m| m.category.last().unwrap().to_string())
                            .collect(),
                    )
                },
                mono,
            ),
            MonoResult::Naikedo(nai, aru) => {
                Result::Message((chara.kaeshi.toikake.aru.naikedo)(&nai, &aru.join("か\n")))
            }
            MonoResult::Wakaran(donna, mono) => {
                Result::Message((chara.kaeshi.toikake.aru.wakaran)(&donna, &mono))
            }
            MonoResult::Nai(mono) => Result::Message((chara.kaeshi.toikake.aru.nai)(&mono)),
        };
    }
}
