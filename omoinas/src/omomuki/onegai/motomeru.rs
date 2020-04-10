use crate::cotoha;
use crate::hitogata;
use crate::omomuki;
use crate::Tumori;

#[derive(Clone, Debug)]
pub struct Motomeru {
    pub nani: Option<omomuki::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if if let Some(d) = &omomuki.doushita {
        d.suru == "くださる"
    } else {
        false
    } {
        return Some(Box::new(Motomeru {
            nani: omomuki.nani.clone(),
        }));
    };

    if if let Some(d) = &omomuki.donoyouni {
        d == "欲しい" || d == "ほしい"
    } else {
        false
    } {
        return Some(Box::new(Motomeru {
            nani: omomuki.nani.clone(),
        }));
    };
    return None;
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
