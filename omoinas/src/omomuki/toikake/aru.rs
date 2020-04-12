use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
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
            if vec!["ある", "くれる", "貰える"].contains(&suru.doushita.suru.as_str()) {
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
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return if let Some(nani) = &self.nani {
            match mono::get_mono(&nani.mono, &nani.donna) {
                MonoResult::Category(category) => (chara.kaeshi.toikake.aru.iroiro)(category),
                MonoResult::Mono(mono) => {
                    if mono.len() == 1 {
                        (chara.kaeshi.toikake.aru.aru)(&mono[0].name)
                    } else {
                        (chara.kaeshi.toikake.aru.iroiro)(
                            mono.iter().map(|m| m.name.as_str()).collect(),
                        )
                    }
                }
                MonoResult::Kawari(nai, aru) => (chara.kaeshi.toikake.aru.naikedo)(
                    &nai,
                    &aru.iter()
                        .map(|m| m.name.as_str())
                        .collect::<Vec<&str>>()
                        .join("か"),
                ),
                MonoResult::Wakaran(donna, mono) => {
                    (chara.kaeshi.toikake.aru.wakaran)(&donna, &mono)
                }
                MonoResult::Nai(mono) => (chara.kaeshi.toikake.aru.aru)(&mono),
            }
        } else {
            // 何かない?
            (chara.kaeshi.toikake.aru.iroiro)(mono::get_menu())
        };
    }
}
