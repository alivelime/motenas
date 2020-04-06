use crate::cotoha;
use crate::model::mono;
use crate::omomuki;
use crate::Tumori;

pub struct Motomeru {
    nani: Option<omomuki::Nani>,
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
    fn get_kotae(&self) -> String {
        if let Some(nani) = &self.nani {
            return mono::get_mono(&nani.mono.clone(), &nani.donna.clone());
        } else {
            // 何かない?
            return String::from("お茶かコーヒーがあるよ");
        }
    }
}
