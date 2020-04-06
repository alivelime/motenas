use crate::cotoha;
use crate::model::mono;
use crate::omomuki;
use crate::Tumori;

pub struct Aru {
    nani: Option<omomuki::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    if if let Some(d) = &omomuki.doushita {
        d.suru == "ある"
    } else {
        false
    } {
        return Some(Box::new(Aru {
            nani: omomuki.nani.clone(),
        }));
    };

    return None;
}

impl Tumori for Aru {
    fn get_kotae(&self) -> String {
        if let Some(nani) = &self.nani {
            return mono::get_mono(&nani.mono, &nani.donna);
        } else {
            // 何かない?
            return String::from("お茶かコーヒーがあるよ");
        }
    }
}
