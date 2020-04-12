use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Motomeru {
    pub nani: Option<omomuki::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    return match omomuki {
        omomuki::Omomuki::Suru(suru) => {
            if suru.doushita.suru == "下さる" {
                Some(Box::new(Motomeru {
                    nani: suru.nani.clone(),
                }))
            } else {
                None
            }
        }
        omomuki::Omomuki::Keiyou(keiyou) => {
            if vec!["欲しい", "ほしい"].contains(&keiyou.dou.as_str()) {
                Some(Box::new(Motomeru {
                    nani: keiyou.nani.clone(),
                }))
            } else {
                None
            }
        }
        omomuki::Omomuki::Taigen(taigen) => {
            if vec!["頂戴", "ちょうだい"].contains(&taigen.suru.as_str()) {
                Some(Box::new(Motomeru {
                    nani: taigen.nani.clone(),
                }))
            } else {
                None
            }
        }
        _ => None,
    };
}

impl Tumori for Motomeru {
    fn kotafu(&self) -> Box<dyn Tumori> {
        return Box::new(omomuki::toikake::aru::Aru {
            nani: self.nani.clone(),
        });
    }
    fn get_kotae(&self, chara: &hitogata::Hitogata) -> String {
        return (chara.kaeshi.error.noimpl)();
    }
}
