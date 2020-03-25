use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Neru {}

impl Neru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai()
            && cotoha::has_lemma(chunks, vec!["おやすみ", "おやすみなさい"]).is_some()
        {
            return Some(Box::new(Neru {}));
        }
        if omomuki.dare.is_none()
            && if let Some(d) = &omomuki.doushita {
                d.suru == "寝る" && d.toki == omomuki::Toki::Ima
            } else {
                false
            }
        {
            return Some(Box::new(Neru {}));
        }
        return None;
    }
}
impl Tumori for Neru {
    fn get_kotae(&self) -> String {
        return String::from("はい、おやすみ");
    }
}
