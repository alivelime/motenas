use crate::model::hitogata::Hitogata;
use crate::model::kotoba::Nani;
use crate::model::mono::MonoResult;
use crate::omomuki::{self, Result};
use crate::repository::mono;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Aru {
    pub nani: Vec<Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if suru.doushita.suru == "ある" && suru.hatena {
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
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(self.clone());
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        let nani = self
            .nani
            .iter()
            .find(|n| !n.has(vec!["ばあちゃん", "おばあちゃん", "何か", "何"]));
        return match mono::get_mono(&chara.omise, nani) {
            MonoResult::Category(category) => {
                Result::Message((chara.kaeshi.toikake.aru.iroiro)(category))
            }
            MonoResult::Mono(mono, category) => match mono.len() {
                1 => Result::Mono(
                    (chara.kaeshi.toikake.aru.aru)(mono[0].category.last().unwrap().as_str()),
                    mono,
                ),
                2..=7 => Result::Mono(
                    (chara.kaeshi.toikake.aru.iroiro)(
                        mono.iter()
                            .map(|m| m.category.last().unwrap().to_string())
                            .collect(),
                    ),
                    mono,
                ),
                _ => Result::Message((chara.kaeshi.toikake.aru.iroiro)(category)),
            },
            MonoResult::Naikedo(nai, nara, aru_mono, aru_category) => match aru_mono.len() {
                1..=7 => Result::Mono(
                    (chara.kaeshi.toikake.aru.naikedo)(&nai, &nara, &aru_category.join("か\n")),
                    aru_mono,
                ),
                _ => Result::Message((chara.kaeshi.toikake.aru.naikedo)(
                    &nai,
                    &nara,
                    &aru_category.join("か\n"),
                )),
            },
            MonoResult::Nai(mono) => Result::Message((chara.kaeshi.toikake.aru.nai)(&mono.namae())),
        };
    }
}
