use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Dekakeru {
    doko: Option<String>,
    nani: Option<omomuki::Nani>,
}

pub fn new(omomuki: &omomuki::Omomuki, _: &cotoha::ParseObjects) -> Option<Box<dyn Tumori>> {
    // TODO して来る
    if omomuki.dare.is_none()
        && if let Some(d) = &omomuki.doushita {
            d.suru == "行く" && d.toki == omomuki::Toki::Ima
        } else {
            false
        }
    {
        let mut n = omomuki.nani.clone();
        if if let Some(nani) = &omomuki.nani {
            nani.mono == "お昼"
        } else {
            false
        } {
            n = Some(omomuki::Nani {
                mono: String::from("お昼ご飯食べ"),
                donna: None,
            });
        }
        return Some(Box::new(Dekakeru {
            doko: omomuki.doko.clone(),
            nani: n,
        }));
    }

    return None;
}

impl Tumori for Dekakeru {
    fn get_kotae(&self) -> String {
        return if let Some(doko) = &self.doko {
            format!("おや、{}に行くのかい。\n気をつけて", doko)
        } else if let Some(nani) = &self.nani {
            format!("おや、{}に行くのかい。\n行ってらっしゃい", nani.mono)
        } else {
            String::from("はい、行ってらっしゃい")
        };
    }
}
