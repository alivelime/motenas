use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Modoru {}

impl Modoru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        // 行ってきた
        if omomuki.dare == None
            && omomuki.doko == None
            && if let Some(d) = &omomuki.doushita {
                (d.suru == "戻る" || d.suru == "帰る") && d.toki == omomuki::Toki::Mukashi
            } else {
                false
            }
        {
            return Some(Box::new(Modoru {}));
        }
        if omomuki.is_tawaimo_nai() && cotoha::has_lemma(chunks, vec!["ただいま"]).is_some() {
            return Some(Box::new(Modoru {}));
        }
        return None;
    }
}
impl Tumori for Modoru {
    fn get_kotae(&self) -> String {
        return String::from("はい、おかえり");
    }
}
