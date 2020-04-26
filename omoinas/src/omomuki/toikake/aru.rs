use crate::hitogata;
use crate::model;
use crate::omomuki::{self, Result};
use crate::repository::mono;
use crate::Tumori;

use crate::model::mono::MonoResult;

#[derive(Clone, Debug)]
pub struct Aru {
    pub nani: Option<model::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if vec!["ある"].contains(&suru.doushita.suru.as_str()) && suru.hatena {
                return Some(Box::new(Aru {
                    nani: suru.nani.clone(),
                }));
            }
        }
        omomuki::Omomuki::Keiyou(keiyou) => {
            if keiyou.dou == "ない" && keiyou.hatena {
                return Some(Box::new(Aru {
                    nani: keiyou.nani.clone(),
                }));
            }
        }
        _ => {}
    };
    return None;
}

impl Tumori for Aru {
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
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
            MonoResult::Naikedo(nai, nara, aru) => {
                Result::Message((chara.kaeshi.toikake.aru.naikedo)(
                    &nai,
                    &nara,
                    &aru.join("か\n"),
                ))
            }
            MonoResult::Nai(mono) => Result::Message((chara.kaeshi.toikake.aru.nai)(&mono.namae())),
        };
    }
}
