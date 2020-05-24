use crate::model::hitogata::Hitogata;
use crate::model::kotoba::{Koto, Kotoba, Nani};
use crate::omomuki::{self, Omomuki, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Motomeru {
    pub nani: Vec<Nani>,
}

pub fn new(omomuki: &Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        Omomuki::Suru(suru) => {
            if Kotoba::from_vec(vec!["くれる", "貰える"]) == suru.doushita.suru {
                return Some(Box::new(Motomeru {
                    nani: suru.nani.clone(),
                }));
            }
            if suru.nani.iter().any(|n| n.has(vec!["お腹", "腹"]))
                && Kotoba::from_vec(vec!["空く", "すく", "減る"]) == suru.doushita.suru
            {
                return Some(Box::new(Motomeru {
                    nani: vec![Nani {
                        mono: vec![Koto::from_str("食べ物")],
                        donna: None,
                    }],
                }));
            }
            if suru
                .nani
                .iter()
                .any(|n| n.has(vec!["のど", "喉", "のどか"]))
                && Kotoba::from_vec(vec!["かわく", "乾く", "渇く", "わく"]) == suru.doushita.suru
            {
                return Some(Box::new(Motomeru {
                    nani: vec![Nani {
                        mono: vec![Koto::from_str("ソフトドリンク")],
                        donna: None,
                    }],
                }));
            }
        }
        Omomuki::Shitai(shitai) => match shitai.doushita.suru.as_str() {
            "食べる" => {
                return Some(Box::new(Motomeru {
                    nani: match shitai.nani.len() {
                        0 => vec![Nani {
                            mono: vec![Koto::from_str("食べ物")],
                            donna: None,
                        }],
                        _ => shitai.nani.clone(),
                    },
                }))
            }
            "飲む" => {
                return Some(Box::new(Motomeru {
                    nani: match shitai.nani.len() {
                        0 => vec![Nani {
                            mono: vec![Koto::from_str("飲み物")],
                            donna: None,
                        }],
                        _ => shitai.nani.clone(),
                    },
                }))
            }
            "読む" => {
                return Some(Box::new(Motomeru {
                    nani: match shitai.nani.len() {
                        0 => vec![Nani {
                            mono: vec![Koto::from_str("読み物")],
                            donna: None,
                        }],
                        _ => shitai.nani.clone(),
                    },
                }))
            }
            _ => {}
        },
        Omomuki::Shite(suru) => {
            if Kotoba::from_vec(vec!["下さる", "見せる", "くれる", "寄越す", "貸す"])
                == suru.doushita.suru
            {
                return Some(Box::new(Motomeru {
                    nani: suru.nani.clone(),
                }));
            }
        }
        Omomuki::Keiyou(keiyou) => {
            if Kotoba::from_vec(vec!["欲しい", "ほしい"]) == keiyou.dou {
                return Some(Box::new(Motomeru {
                    nani: keiyou.nani.clone(),
                }));
            }
        }
        Omomuki::Taigen(taigen) => {
            if Kotoba::from_vec(vec!["頂戴", "ちょうだい"]) == taigen.suru {
                return Some(Box::new(Motomeru {
                    nani: taigen.nani.clone(),
                }));
            }
        }
        _ => {}
    };
    return None;
}

impl Tumori for Motomeru {
    fn kotafu(&self, _: &Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::toikake::aru::Aru {
            nani: self.nani.clone(),
        });
    }
    fn get_kotae(&self, chara: &Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
