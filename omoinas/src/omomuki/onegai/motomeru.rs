use crate::hitogata;
use crate::model;
use crate::omomuki::{self, Result};
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Motomeru {
    pub nani: Option<model::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki) -> Option<Box<dyn Tumori>> {
    match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if vec!["くれる", "貰える"].contains(&suru.doushita.suru.as_str()) {
                return Some(Box::new(Motomeru {
                    nani: suru.nani.clone(),
                }));
            }
            if let Some(nani) = &suru.nani {
                if vec!["お腹", "原"].contains(&nani.namae().as_str())
                    && vec!["空く", "すく", "へる", "減る"].contains(&suru.doushita.suru.as_str())
                {
                    return Some(Box::new(Motomeru {
                        nani: Some(model::Nani {
                            mono: vec![String::from("弁当")],
                            donna: None,
                        }),
                    }));
                }
                if (nani.namae() == "喉"
                    && vec!["かわく", "渇く", "乾く"].contains(&suru.doushita.suru.as_str()))
                    || (nani.namae() == "のどか" && suru.doushita.suru.as_str() == "わく")
                {
                    return Some(Box::new(Motomeru {
                        nani: Some(model::Nani {
                            mono: vec![String::from("ソフトドリンク")],
                            donna: None,
                        }),
                    }));
                }
            }
        }
        omomuki::Omomuki::Shitai(shitai) => match shitai.doushita.suru.as_str() {
            "食べる" => {
                return Some(Box::new(Motomeru {
                    nani: match shitai.nani {
                        Some(_) => shitai.nani.clone(),
                        None => Some(model::Nani {
                            mono: vec![String::from("食べ物")],
                            donna: None,
                        }),
                    },
                }))
            }
            "飲む" => {
                return Some(Box::new(Motomeru {
                    nani: match shitai.nani {
                        Some(_) => shitai.nani.clone(),
                        None => Some(model::Nani {
                            mono: vec![String::from("飲み物")],
                            donna: None,
                        }),
                    },
                }))
            }
            "読む" => {
                return Some(Box::new(Motomeru {
                    nani: match shitai.nani {
                        Some(_) => shitai.nani.clone(),
                        None => Some(model::Nani {
                            mono: vec![String::from("読み物")],
                            donna: None,
                        }),
                    },
                }))
            }
            _ => {}
        },
        omomuki::Omomuki::Shite(suru) => {
            if vec!["下さる", "見せる"].contains(&suru.doushita.suru.as_str()) {
                return Some(Box::new(Motomeru {
                    nani: suru.nani.clone(),
                }));
            }
            if vec!["くれる", "寄越す"].contains(&suru.doushita.suru.as_str()) {
                return Some(Box::new(Motomeru {
                    nani: suru.nani.clone(),
                }));
            }
        }
        omomuki::Omomuki::Keiyou(keiyou) => {
            if vec!["欲しい", "ほしい"].contains(&keiyou.dou.as_str()) {
                return Some(Box::new(Motomeru {
                    nani: keiyou.nani.clone(),
                }));
            }
        }
        omomuki::Omomuki::Taigen(taigen) => {
            if vec!["頂戴", "ちょうだい"].contains(&taigen.suru.as_str()) {
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
    fn kotafu(&self, _: &hitogata::Hitogata) -> Box<dyn Tumori> {
        return Box::new(omomuki::toikake::aru::Aru {
            nani: self.nani.clone(),
        });
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> Result {
        return Result::Message((chara.kaeshi.error.noimpl)());
    }
}
