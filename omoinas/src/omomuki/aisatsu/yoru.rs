use crate::cotoha;
use crate::omomuki;
use crate::Tumori;

pub struct Yoru {}

impl Yoru {
    pub fn new(
        omomuki: &omomuki::Omomuki,
        chunks: &Vec<cotoha::ParseObject>,
    ) -> Option<Box<dyn Tumori>> {
        if omomuki.is_tawaimo_nai() && cotoha::has_lemma(chunks, vec!["こんばんは"]).is_some()
        {
            return Some(Box::new(Yoru {}));
        }
        return None;
    }
}
impl Tumori for Yoru {
    fn get_kotae(&self) -> String {
        return String::from("はい、こんばんは");
    }
}
