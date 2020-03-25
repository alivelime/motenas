use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Dekakeru {
    omomuki: omomuki::Omomuki,
}

impl Dekakeru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        _: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        // TODO して来る
        if omomuki.dare.is_none()
            && if let Some(d) = &omomuki.doushita {
                d.suru == "行く" && d.toki == omomuki::Toki::Ima
            } else {
                false
            }
        {
            let mut n = omomuki.clone();
            if omomuki.itsu.as_ref().unwrap_or(&String::from("")) == "お昼"
                && omomuki.nani.is_none()
                && omomuki.doko.is_none()
            {
                n.nani = Some(String::from("お昼ご飯食べ"));
            }
            return Some(Box::new(Dekakeru { omomuki: n }));
        }

        return None;
    }
}
impl Tumori for Dekakeru {
    fn get_kotae(&self) -> String {
        return if let Some(doko) = &self.omomuki.doko {
            format!("おや、{}に行くのかい。\n気をつけて", doko)
        } else if let Some(nani) = &self.omomuki.nani {
            format!("おや、{}に行くのかい。\n行ってらっしゃい", nani)
        } else {
            String::from("はい、行ってらっしゃい")
        };
    }
}
